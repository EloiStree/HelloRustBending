use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey};
use rsa::{RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::Write;
use std::error::Error;
//use crate::PaddingScheme;

fn main() -> Result<(), Box<dyn Error>> {
    // Generate a 1024-bit RSA private key
    let mut rng = OsRng;
    let bits = 1024;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;

    // Derive the public key from the private key
    let public_key = RsaPublicKey::from(&private_key);

    let line_ending = rsa::pkcs1::LineEnding::LF;


    // Write the private key to a file
    let priv_pem = private_key.to_pkcs1_pem(line_ending)?;
    let mut priv_file = File::create("private_key.pem")?;
    priv_file.write_all(priv_pem.as_bytes())?;

    // Write the public key to a file
    let pub_pem = public_key.to_pkcs1_pem(line_ending)?;
    let mut pub_file = File::create("public_key.pem")?;
    pub_file.write_all(pub_pem.as_bytes())?;

    let private_pem_string = priv_pem.to_string();
    let public_pem_string = pub_pem.to_string();

    println!("private_pem_string: {}", private_pem_string);
    println!("public_pem_string: {}", public_pem_string);

    

    Ok(())
}
