// #![no_main]
// sp1_zkvm::entrypoint!(main);

// Initialize a heteromorphic blob of data.
// Assert that some encrypted data is all
// zeroes.

use chacha::ChaCha;
use chacha::KeyStream;
// use chacha::SeekableKeyStream;

/// We init a chunk of data that is encrypted.
/// We prove initialization to a slice of 32, zero bytes
/// and knowledge of the symmetric encryption key.
pub fn main() {
    let input: Vec<u8> = sp1_zkvm::io::read_vec();
    // the mem_pointer points to unused space in the
    // encryption stream. During operation the last state
    // should be at `mem_pointer - byte_len`, with the next
    // state at `mem_pointer`
    let (key, mem_len, nonce): ([u8; 32], u32, [u8; 12]) =
        bincode::deserialize(&input).expect("failed to deserialize input key");

    // put some state at the beginning of the thing
    let mut chacha = ChaCha::new_ietf(&key, &nonce);

    // first 4 bytes contain length of the data
    // total region length = len_bytes (4) + mem_len
    let mem_pointer = 4 + mem_len;
    let mut data = Vec::with_capacity(mem_pointer.try_into().unwrap());
    data.extend_from_slice(&mut mem_len.to_le_bytes());
    data.resize(mem_pointer.try_into().unwrap(), 0u8);

    let mem_pointer = data.len();

    // encrypt `data` in place
    chacha
        .xor_read(&mut data)
        .expect("failed to read chacha stream");

    // data is now encrypted
    // output the mem_pointer for the encryption stream
    // in heteromorphic programs output for next program
    // will start at `mem_pointer`, with previous output
    // at `mem_pointer - mem_len`
    let out = bincode::serialize(&(nonce, mem_pointer, nonce)).expect("failed to serialize output");
    sp1_zkvm::io::commit_slice(&out);
}
