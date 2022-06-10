**ZT - BLOCKCAIN MANUAL**
## Blocks and Hashing
### Cryptocurrency Blockchains
**Two main data structures**
- The blocks in the blockchain
- The transactions within the blocks

**Anclillary data**
- Wallets
- Addresses
- Balances
- Peers
### Gneric Blockchains (With PoW support)
Blockchain = chronological, sequential list of blocks

**Blocks contain this information:**
- Index: this block's location within the list of blocks
- Payload: any relevant information or events that have occurred for/in block
- Timestamp: give our blockchain a sense of tiem
- Nonce: special number used for mining (PoW verification)
- Previous block hash: cryptographic fingerprint of previous block
- Hash: cryptographic fingerprint of all of the above data concatenated together
### What is Hashing ?
In a nutshell, a hash algorithm consists of a set of irreversible computations that can be performed on a datum to generate a (usually) unique byte sequence.
The best to use in my case is: [SHA-256](https://es.wikipedia.org/wiki/SHA-2)



### **Content**
- [RUST_MANUAL](https://doc.rust-lang.org/1.30.0/book/2018-edition/foreword.html)
