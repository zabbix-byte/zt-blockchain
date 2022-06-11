**⚛ ZT - BLOCKCAIN MANUAL**
## Blocks and Hashing 💱
### Cryptocurrency Blockchains 
**Two main data structures**
- The blocks in the blockchain
- The transactions within the blocks

**Anclillary data**
- Wallets
- Addresses
- Balances
- Peers
### Gneric Blockchains (With PoW support)  ⛏️
Blockchain = chronological, sequential list of blocks

**Blocks contain this information:**
- Index: this block's location within the list of blocks
- Payload: any relevant information or events that have occurred for/in block
- Timestamp: give our blockchain a sense of tiem
- Nonce: special number used for mining (PoW verification)
- Previous block hash: cryptographic fingerprint of previous block
- Hash: cryptographic fingerprint of all of the above data concatenated together

###  What is Hashing ? 🔒
In a nutshell, a hash algorithm consists of a set of irreversible computations that can be performed on a datum to generate a (usually) unique byte sequence.
The best to use in my case is: [SHA-256](https://es.wikipedia.org/wiki/SHA-2)

###  What is Difficuty ? 🔑
SHA-256 generates a 32-bytes hash. Difficulty (in our case) specifies the unsigned 128-bit
integer value that the most significant 16 bytes of the hash of a block must be less than before it is consider "valid" (if those bytes are interpreted as a single number insted of a series of bytes). Difficulty will be stored as a field of the block struct.

Difficulty could also be expressed as:
- Te first n bytes of the hash that must be zero
- Te number of bits or bytes at the beginning of the hash that must be 0

### Nonce


### **⚛ Content**
- [Rust manuals](https://doc.rust-lang.org/1.30.0/book/2018-edition/foreword.html)
- [Difficulty](https://en.bitcoin.it/wiki/Difficulty)
- [Hex](https://en.wikipedia.org/wiki/Hexadecimal)