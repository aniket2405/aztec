extern crate miracl_core;
extern crate miracl_bls;

use miracl_core::rand::RAND;
use miracl_core::bls12381::{PAIR, G1_POINT_ORDER, G2_POINT_ORDER};
use miracl_bls::{bls, bls12381::{G1, G2}};

fn main() {
    // Generate a private key
    let mut rng = RAND::os_time();
    let mut sk = G2::new();
    sk.random(&mut rng);
    
    // Generate a public key
    let mut pk = G1::new();
    PAIR::g2mul(&sk, &mut pk);
    
    // Sign a message
    let msg = "hello world".as_bytes();
    let mut sig = G1::new();
    bls::core::sign(&sk, msg, &mut sig);
    
    // Verify the signature
    let mut pks = vec![pk];
    let mut msgs = vec![msg.to_vec()];
    let mut sigs = vec![sig];
    let valid = bls::core::verify(&pks, &msgs, &sigs);
    
    if valid {
        println!("Signature is valid!");
    } else {
        println!("Invalid signature.");
    }
}

// This code generates a BLS private key and public key, signs a message using the private key, 
// and then verifies the signature using the public key.
