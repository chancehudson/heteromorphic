# heteromorphic encryption

Read some encrypted data in zk, decrypt, modify, and re-encrypt as output. Trustless mutation of encrypted state.

## byte storage in an encryption space

Consider a chacha stream of length `2**64` bytes = 17383583047 GB. This represents a quantum storage space. Commiting two different values to the same location in the encryption space compromises the private key, breaking encryption.

ZK programs can trustlessly read and write bytes in the encryption space.

A step system first comes to mind for storing indexed versions of a finite buffer of bytes.

The encryption space is encoded like so:
- final 4 bytes: buffer size
- byte offset % buffer size: buffer at version

**ZK proofs can be used to enforce and exploit structure in an encryption space**

Buffers are committed at the specified index, forming a key-value map. Values are accessible in `O(1)` by seeking to a point in the encryption stream.

An example application might use zk to mutate buffers at predetermined indices based on logic involving buffers at other indices.

Storing and indexing data is trivial because it's a byte stream (though random access complicates this slightly). Contrast this to the complexity of maintaining/operating merkle trees.

### concrete example: sequential write system
We configure the stream:
- `mem_len`: byte length of the memory available for reading and writing

Now we split the stream into chunks of `mem_len` bytes, each chunk representing a historical state. We mutate the state using a zk proof to write changes to the output stream at the latest byte offset, committing to values. We also output the new memory pointer location: where the next mutation will be written in the encryption stream.

NIT: memory is the wrong term, it's more like storage. The next step is sharding the encryption space so the storage is delta compressed by seeking around the stream. Mentally I'm modeling it as a byte storage.

## practical implications

We arrive conceptually at **encryption spaces as block storage devices**. We can then run ZK programs in the encryption space.

In ecdsa a `[u8; 32]` represents a keypair. In chacha a `[u8; 32]` represents a **quantum storage space**. Writing two different pieces of data to the same location in the encryption space compromises the private key. Encryption spaces have a _finite_ amount of usable storage for a given private key.

Hosting the storage means storing the full encryption transcript, and the _latest_ zk proof. So storage size is constant and trustless to the data being stored (1).

1: achieving trustless random reads will require an embedded hashchain system

## why use encrypted data and not hashes? hashes are constant size after all

committing to hashes creates two pieces of information to be managed: the hash and the data. As a result the hash must be checked against the data and complexity is introduced.

If we commit via encryption we can use a stream cipher to selectively decrypt and mutate regions of the committed data. Arbitrary memory structures can be designed based on access needs.

The structure is inherently copy-on-write because re-using a nonce compromises the private key. It might be considered a quantum storage device, committing to a value means not committing to another value without revealing the private key.

## merkle trees in an encryption space

it may be possible to embed a merkle tree in a kv structure in an encryption space. Using some sort of pointer indirection for memory slots.
