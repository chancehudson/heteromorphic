# heteromorphic encryption

Read some encrypted data in zk, decrypt, modify, and re-encrypt as output. Trustless mutation of encrypted state.

## Encrypted spaces

Consider a chacha stream of length `2**64` bytes = 17383583047 GB. Each byte is committed in the transcript (the full encrypted output of all `2**64` bytes). e.g. **encrypted output proven to be encrypted with a certain key is a commitment to a region in a data stream for said key.** This can be used for version control of data with `O(1)` access to any historically known point.

This is comparable to a merkle tree, without the complexity of a tree structure. Checksumming the stream is more difficult, though with zk proofs we can make maintain a hashchain versioned in the encryption stream.

We configure the stream:
- `mem_len`: byte length of the memory available for reading and writing

Now we split the stream into chunks of `mem_len` bytes, each chunk representing a historical state. We mutate the state using a zk proof to write changes to the output stream at the latest byte offset, committing to values. We also output the new memory pointer location: where the next mutation will be written in the encryption stream.



