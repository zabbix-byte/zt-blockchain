### 🐧 ZT-BLOCKCHAIN PROJECT 🐧
#### 🛸 Current hash output
```rs
Verify: true
Mined block: Block[1]: 3f3b19044d2488ee52e45207934593130723e563670db0cc4f89556683cd0a00 at: 1655210489815 whith: Test block nonce: 678
Verify: true
Mined block: Block[2]: 0c0f25c310075844cb2279615ecd17ddccbb7296d900bb3c8ad4b45bff340c00 at: 1655210489816 whith: Test block nonce: 376
Verify: true
Mined block: Block[3]: 80fab3112773535ea2f2cf22d27cf3ef6b5454797b36066e3b0509ac04710d00 at: 1655210489817 whith: Test block nonce: 9408
Verify: true
Mined block: Block[4]: 43f0a09f000c2df3e5c821c72bb86f13c0e9db3f1b3f74c4bb4dce70f7930f00 at: 1655210489836 whith: Test block nonce: 4051
Verify: true
Mined block: Block[5]: e65248fbb6834604f849a031095c671b92a6bf698d11b52341b7e291d9c30800 at: 1655210489844 whith: Test block nonce: 638
Verify: true
Mined block: Block[6]: d93dd4faac17bfab9641418523452c4141e9c7b4551811e1771c3f4737550400 at: 1655210489846 whith: Test block nonce: 706
Verify: true
Mined block: Block[7]: 2363786cd1ad70bacce91f1b2d319bb826a4b118f21a9d9ecde72ea78c5a0b00 at: 1655210489847 whith: Test block nonce: 6138
Verify: true
Mined block: Block[8]: ffaa7cd7c1557e0bc3707a55862ce4814309c4600b87afc67808ffa904590b00 at: 1655210489860 whith: Test block nonce: 4443
Verify: true
Mined block: Block[9]: 4276116ee9361b50ea557c3109ab8cfec27aacc3cab2bbcddc2381aa2e8c0400 at: 1655210489871 whith: Test block nonce: 4798
Verify: true
Mined block: Block[10]: 4e376ecd92a4829d585eef290e28e4b6412ca74f83c8fb90e530d4d030f50c00 at: 1655210489882 whith: Test block nonce: 8327
```

####  🔩 Dependencies
- To install openssl-dev [go](https://docs.rs/crate/openssl/0.9.24) (needed for crypto-hash)
- [hex](https://github.com/sitkevij/hex) = "0.3.2"
- [crypto-hash](https://github.com/RustCrypto/hashes) = "0.3.4"

## 🐧**Blockchain manual**🐧
### Blocks and Hashing 💱
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

###  Nonce 🐧
A hash is a unique, reproducible fingerprint for some data. Therefore, to make a "valid" hash(per difficulty), we must somehow change the bytes we send to the function (the pre-image).
Remember that even one small change to the input changes the resultant hash drastically. This effect is commonly called avalanching.

###  Block Verification ✅
Given the implementation we have so far, we can also implement a few rudimentary block verfication tets. These steps would be executed wherenever we recibe a new block from a peer. [BITCOIN](https://www.researchgate.net/publication/283622936_Modeling_and_Verification_of_the_Bitcoin_Protocol)

### **⚛ Content**
- [Rust manuals](https://doc.rust-lang.org/1.30.0/book/2018-edition/foreword.html)
- [Difficulty](https://en.bitcoin.it/wiki/Difficulty)
- [Hex](https://en.wikipedia.org/wiki/Hexadecimal)
