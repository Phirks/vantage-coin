use chrono::prelude::*;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey, Signature};
use sha2::{Digest, Sha256};
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
        let mut block = Block {
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
        };
        block.hash = Block::calculate_hash(&block);
        block
    }
    /// controls how the hash is generated, this version usese the timestamp, data, and previous hash to randomize.
    fn calculate_hash(block: &Block) -> String {
        let mut hasher = Sha256::new();
        hasher.update(block.timestamp.to_string().as_bytes());
        hasher.update(&block.data.as_bytes());
        hasher.update(&block.previous_hash.as_bytes());
        let hash = hasher.finalize();
        let mut hash_str = String::new();
        for byte in hash {
            write!(&mut hash_str, "{:02x}", byte).unwrap();
        }
        hash_str
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
    fn sign(&mut self, secret_key: &SecretKey) {
        let context = Secp256k1::new();
        let message = self.create_message();
        let signature = context.sign(&Message::from_slice(&message).unwrap(), secret_key);

        self.signature = signature.to_string();
    }
    fn create_message(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.sender.as_bytes());
        bytes.extend(self.recipient.as_bytes());
        bytes.extend(&self.amount.to_le_bytes());
        bytes
    }
    //Check the signature of the transaction is correct
    fn verify(&self, public_key: &PublicKey) -> bool {
        let context = Secp256k1::new();
        let message = self.create_message();
        let signature = Signature::from_str(&self.signature).unwrap();

        context
            .verify(
                &Message::from_slice(&message).unwrap(),
                &signature,
                public_key,
            )
            .is_ok()
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_owned(), String::new());
        Blockchain {
            chain: vec![genesis_block],
        }
    }
    fn add_block(&mut self, data: String) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let new_block = Block::new(Self::current_timestamp(), data, previous_hash);
        self.chain.push(new_block);
    }
    fn current_timestamp() -> i64 {
        Utc::now().timestamp()
    }
    fn verify_and_add_block(&self, block: &Block) -> bool {
        todo!();
    }
}

struct Node<T> {
    blockchain: Blockchain,
    peers: Vec<T>,
}

impl<T: ToSocketAddrs> Node<T> {
    fn new() -> Self {
        todo!();
    }
    async fn sychronize(&self) {
        for peer in &self.peers {
            let mut stream = TcpStream::connect(peer).await.unwrap();
            stream.write_all(b"get_latest_block").await.unwrap();

            let mut buffer = [0; 1024];
            let n = stream.read(&mut buffer).await.unwrap();
            let latest_block = deserialize_block(&buffer[..n]);

            if self.blockchain.verify_and_add_block(latest_block) {
                println!("Block added to blockchain");
            }
        }
    }
    async fn broadcast_new_block(&self, block: Block) {
        let serialized_block = serialize_block(&block);
        for peer in &self.peers {
            let mut stream = TcpStream::connect(peer).await.unwrap();
            stream.write_all(&serialized_block).await.unwrap();
        }
    }
}

fn deserialize_block(buffer: &[u8]) -> &Block {
    todo!();
}
fn serialize_block(block: &Block) -> Vec<u8> {
    todo!();
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                let n = socket.read(&mut buf).await.unwrap();
                if n == 0 {
                    break;
                }
                socket.write_all(&buf[0..n]).await.unwrap();
            }
        });
    }
}
