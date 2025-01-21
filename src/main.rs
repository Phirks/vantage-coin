use chrono::prelude::*;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey, Signature};
use sha2::{Digest, Sha256};
use std::net::SocketAddrV4;
use std::{fmt::Write, str::FromStr}; //encryption crate //stdout stream access
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

#[derive(Debug)]
/// A block structure for containing transactions
struct Block {
    /// time stamp pulled from system time
    timestamp: i64,
    /// data contained in the block
    data: String,
    /// the hash from the previous block
    previous_hash: String,
    /// the hasf generated for this block
    hash: String,
}

impl Block {
    /// create a new block, generates a new hash based on the block data
    fn new(timestamp: i64, data: String, previous_hash: String) -> Self {
        // create a mutable block from inputs
        let mut block = Block {
            timestamp, // the time the block is made, from  Utc::now().timestamp()
            data, // data inside the block, first one is a string saying it's the genesis block
            previous_hash, // based on the last block, hashed with SHA256
            hash: String::new(), //the SHA256 hash for this block, about to be created
        };
        block.hash = Block::calculate_hash(&block); // hash the block as it is and add that hash to it.
        block  //return the block with the new hash
    }
    /// controls how the hash is generated, this version usese the timestamp, data, and previous hash to randomize.
    fn calculate_hash(block: &Block) -> String {
        let mut hasher = Sha256::new(); // create an empty SHA256 
        hasher.update(block.timestamp.to_string().as_bytes()); //add data from the timestamp to the data to be hashed
        hasher.update(&block.data.as_bytes()); // add data from the block data to the data to be hashed
        hasher.update(&block.previous_hash.as_bytes()); //add data from the block before to the data to be hashed
        let hash = hasher.finalize(); // actually do the hashing based on the previously added data
        let mut hash_str = String::new();  // make a string to store the hash
        for byte in hash { // go byte by byte through the hash
            write!(&mut hash_str, "{:02x}", byte).unwrap(); // puts each byte of the hash formatted like "0x0a"
        }
        hash_str //return the hash string
    }
}

///single transaction struct, sent as a message signed with private key
struct Transaction {
    sender: String,
    recipient: String,
    amount: f32,
    signature: String,
}

impl Transaction {
    ///add signature that proves ownership of the private key
    fn sign(&mut self, secret_key: &SecretKey) {
        let context = Secp256k1::new(); // create an object to hold a signature
        let message = self.create_message();  // take the transaction as a vec<u8> and store it for signing as bytes
        let signature = context.sign(&Message::from_slice(&message).unwrap(), secret_key);  // take the vec<u8> and sign it using the secret key
        self.signature = signature.to_string(); // store the signature in the relevant field
    }
    fn create_message(&self) -> Vec<u8> {
        let mut bytes = Vec::new(); // holder for byte vector to store transaction data
        bytes.extend(self.sender.as_bytes());  // grab the sender
        bytes.extend(self.recipient.as_bytes()); // grab the recipient
        bytes.extend(&self.amount.to_le_bytes());  //grab the bytes
        bytes // send back the combined bytes as a long byte vector to be hashed as a signature
    }
    //Check the signature of the transaction is correct
    fn verify(&self, public_key: &PublicKey) -> bool {

        let context = Secp256k1::new(); // holder for a signature object
        let message = self.create_message();  // creates a byte array based on transaction data
        let signature = Signature::from_str(&self.signature).unwrap(); // grabs the signature from the transaction

        context
            .verify(
                &Message::from_slice(&message).unwrap(),
                &signature,
                public_key,
            )
            .is_ok() //the .verify() function checks the signature against the public key.
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>, // a blockchain is just a list of blocks in order, where the new one gets added onto the end
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_owned(), String::new()); // creae a first block with dummy data
        Blockchain {
            chain: vec![genesis_block], //return a blockchain with just the genesis block
        }
    }
    fn add_block(&mut self, data: String) {
        let previous_hash = self.chain.last().unwrap().hash.clone(); // grab the last block's hash
        let new_block = Block::new(Self::current_timestamp(), data, previous_hash);  // using the hash, timestamp, and the provided data, make a new block.
        self.chain.push(new_block); // send the new block to the blockchain.
    }
    fn current_timestamp() -> i64 {
        Utc::now().timestamp() // basic timestamp object representing the time/date it was created
    }
    fn verify_and_add_block(&self, block: &Block) -> bool {
        todo!();  // function for verifying block before adding it.
    }
}

struct Node<T> {  // this is a machine acting as an blockchain access point.
    blockchain: Blockchain,  // the universally agreed upon blockchain
    peers: Vec<T>,  // a list of users
}

impl<T: ToSocketAddrs> Node<T> {
    fn new() -> Self {  // create a new node
        todo!(); // need to handle creating a node.
    }
    async fn sychronize(&self) {
        for peer in &self.peers {  // walk through each user
            let mut stream = TcpStream::connect(peer).await.unwrap(); // grab the TCP stream connected to the peer
            stream.write_all(b"get_latest_block").await.unwrap(); // write command to get latest block to TCP stream
            let mut buffer = [0; 1024];  // creates a mutable buffer 1024 bytes long
            let n = stream.read(&mut buffer).await.unwrap(); // fills the bufer with the data in the stream
            let latest_block = deserialize_block(&buffer[..n]);  // ?

            if self.blockchain.verify_and_add_block(latest_block) { // check the block and add it to the blockchain
                println!("Block added to blockchain");  // 
            }
        }
    }
    async fn broadcast_new_block(&self, block: Block) {
        let serialized_block = serialize_block(&block);  // ?
        for peer in &self.peers { // walk through each user
            let mut stream = TcpStream::connect(peer).await.unwrap();  // connect to the peer
            stream.write_all(&serialized_block).await.unwrap(); // send them the latest block
        }
    }
}

fn deserialize_block(buffer: &[u8]) -> &Block {
    todo!();
}
fn serialize_block(block: &Block) -> Vec<u8> {
    todo!();
}

fn main() {
    let node: Node<SocketAddrV4> = Node::new();
    let mut blockchain = node.blockchain;
    blockchain.add_block("data".to_string());

}

// #[tokio::main]
// async fn main() {
//     let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap(); // listen in on 7878 port
//     loop {
//         let (mut socket, _) = listener.accept().await.unwrap(); // get the socket as an object
//         tokio::spawn(async move {  // create a new thread 
//             let mut buf = vec![0; 1024]; // open up a buffer to recieve a signal from the listener
//             loop { // loop through the socket grabbing bytes until it's empty
//                 let n = socket.read(&mut buf).await.unwrap(); // put the received signal inside the buffer
//                 if n == 0 { // if the buffer is empty, leave the loop
//                     break;
//                 }
//                 socket.write_all(&buf[0..n]).await.unwrap();  // write the buffer back to the socket?
//             }
//         });
//     }
// }
