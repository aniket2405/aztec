extern crate curve25519_dalek;
extern crate kzg;
extern crate rand;

use curve25519_dalek::scalar::Scalar;
use kzg::{Commitment, Error, KZG};
use rand::RngCore;

fn main() -> Result<(), Error> {
    // Generate a random polynomial of degree 3
    let mut rng = rand::thread_rng();
    let coeffs: Vec<Scalar> = (0..4).map(|_| Scalar::random(&mut rng)).collect();
    
    // Create a KZG object
    let kzg = KZG::new(coeffs.len())?;
    
    // Compute the polynomial commitment
    let comm = kzg.commit(&coeffs)?;
    
    // Evaluate the polynomial at a point
    let x = Scalar::random(&mut rng);
    let y = kzg.eval_poly(&coeffs, x);
    
    // Verify the polynomial commitment
    let proof = kzg.gen_proof(&coeffs, x)?;
    let valid = kzg.verify_proof(&comm, &proof, x, y)?;
    
    if valid {
        println!("Polynomial commitment is valid!");
    } else {
        println!("Invalid polynomial commitment.");
    }
    
    Ok(())
}

  
<!-- This code generates a random polynomial of degree 3, computes a polynomial commitment using the KZG scheme, 
  evaluates the polynomial at a random point, and then verifies the commitment using a zero-knowledge proof.    -->
