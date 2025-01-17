///! A simple proof of work implementation
/// WARNING: this requires replay protection to be secure
use wasm_bindgen::prelude::*;
use sha2::{Digest,  Sha512};
use hex::{encode};

mod utils;

#[wasm_bindgen]
pub fn test() -> String {

    let h = hash(&HashInputs {
        ts: 2,
        nonce: 1,
        difficulty: 1,
    });
    format!("{:?}", h)
}

/// Hash inputs of timestamp, nonce, and difficulty
#[derive(Default, Debug)]
#[wasm_bindgen]
pub struct HashInputs {
    ts: u64,
    nonce: usize,
    difficulty: u16,
}

/// Scaffolding for WASM bindings
#[wasm_bindgen]
impl HashInputs {
    #[wasm_bindgen(constructor)]
    pub fn new(ts: u64, nonce: usize, difficulty: u16) -> HashInputs {
        HashInputs { ts, nonce, difficulty }
    }

    #[wasm_bindgen(getter)]
    pub fn ts(&self) -> u64 {
        self.ts
    }

    #[wasm_bindgen(getter)]
    pub fn nonce(&self) -> usize {
        self.nonce
    }

    #[wasm_bindgen(getter)]
    pub fn difficulty(&self) -> u16 {
        self.difficulty
    }
}

/// Mines a reward
#[wasm_bindgen]
pub fn mine(ts: u64, difficulty: u16, batch_size: usize) -> Option<HashInputs> {
    if difficulty < 1 {
        panic!("Difficulty must be greater than 0");
    }

    for i in 0..batch_size {
        let hash_inputs = HashInputs {
            ts,
            nonce: i,
            difficulty,
        };
        if verify(&hash_inputs) {
            return Some(hash_inputs);
        }
    }

    None
}

/// Verifies a hash
fn verify(hash_inputs: &HashInputs) -> bool {
    winning_hash(hash(hash_inputs), hash_inputs.difficulty)
}

/// Hashes the hash input
#[wasm_bindgen]
pub fn hash(hash_inputs: &HashInputs) -> String {
    let mut hasher = Sha512::new();
    hasher.update(format!("{}:{}:{}", hash_inputs.ts, hash_inputs.nonce, hash_inputs.difficulty).as_bytes());
    let result = hasher.finalize();
    let intermediate = encode(result);

    let mut hasher = Sha512::new();
    hasher.update(intermediate);
    let result = hasher.finalize();
    encode(result)
}

/// Checks if a hash is a winning hash by checking if the first `difficulty` characters are `0`
fn winning_hash(hash: String, difficulty: u16) -> bool {
    for (i, c) in hash.chars().enumerate() {
        if i >= difficulty as usize {
            break;
        }
        if c != '0' {
            return false;
        }
    }

    true
}
