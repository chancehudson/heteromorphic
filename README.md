# heteromorphic encryption

Read some encrypted data in zk, decrypt, modify, and re-encrypt as output. Trustless mutation of encrypted state.

## byte storage in an encryption stream

Consider a chacha stream of length `2**64` bytes = 17383583047 GB. Each byte is lazily committed in the transcript (set of mutations that occur in zk). e.g. **encrypted output proven to be encrypted with a certain key is a commitment to a region in a data stream for said key.** This can be used for version control of data with `O(1)` access to any historically known point.

This is comparable to a merkle tree, without the complexity of a tree structure. Checksumming the stream is more difficult, though with zk proofs we can make maintain a hashchain version in the memory space (as a sort of kernel/driver).

We configure the stream:
- `mem_len`: byte length of the memory available for reading and writing

Now we split the stream into chunks of `mem_len` bytes, each chunk representing a historical state. We mutate the state using a zk proof to write changes to the output stream at the latest byte offset, committing to values. We also output the new memory pointer location: where the next mutation will be written in the encryption stream.

NIT: memory is the wrong term, it's more like storage. The next step is sharding the encryption space so the storage is delta compressed by seeking around the stream. Mentally I'm modeling it as a byte storage.

## practical implications

We arrive conceptually at **encryption spaces as block storage devices**.

In ecdsa a `[u8; 32]` represents a keypair. In chacha a `[u8; 32]` represents a storage space, provided it can be mutated in zk.

Hosting the storage means storing the full encryption transcript, and the _latest_ zk proof. So storage size is constant and trustless to the data being stored (1).

1: achieving trustless random reads will require an embedded hashchain system
