use openssl::rsa::{Rsa, PrivateKey, PublicKey};
use openssl::hash::{Hasher, MessageDigest};
use openssl::sign::{Signer, Verifier};

fn sign_data(data: &[u8], private_key_xml: &str) -> Result<Vec<u8>, openssl::error::Error> {
    let private_key = PrivateKey::from_pem(private_key_xml.as_bytes())?;
    let rsa = Rsa::from_private_key(&private_key)?;

    let mut signer = Signer::new(MessageDigest::sha256())?;
    signer.set_rsa(&rsa)?;
    signer.update(data)?;

    Ok(signer.sign_pkcs1(&rsa)?)
}

fn verify_signature(data: &[u8], signature: &[u8], public_key_xml: &str) -> Result<bool, openssl::error::Error> {
    let public_key = PublicKey::from_pem(public_key_xml.as_bytes())?;
    let rsa = Rsa::from_public_key(&public_key)?;

    let mut verifier = Verifier::new(MessageDigest::sha256())?;
    verifier.set_rsa(&rsa)?;
    verifier.update(data)?;

    Ok(verifier.verify(signature)?)
}

fn main() {
    // https://github.com/sfackler/rust-openssl#windows
    // YOU need to install vcpkg  on Window to use the code. 
    // So I don't think using openssl is a good idea.

    
    // Example usage (replace with your actual data and keys)
    let data = b"This is the data to be signed";
    let private_key_xml = r#"-----BEGIN RSA PRIVATE KEY-----
MIICXgIBAAKBgQD0VYwAohAQN2P9IBI4zMp1PSg7YjA6EELPBcbxi8bEPADvxf41
lW0ztUC6rSWwPix2iRXkUwuW+yYlhBIkWyiqxkEPMpTNyG1mTzQa+2vUR/hRce0X
p9pHvTZNHT2GkENY4P82oBNcAXOsXeQrKczu+n5tt7JO5BJFsKwFbA1hZQIDAQAB
AoGBAJEVFwvtL4KhkWLln0xQ9ksFZJVWaXKg4zrQoGAfcohlZNz7gUZKE0zn7Kas
V7u+P2KnZfNtCG2/ddhDAw7Jfmw4RS/tNpfS/FuFaXlSNcQGgoPFTlngAgoCszvC
gqTZh5qxXr0S91/S6Rrj02B44K+2qdfmS7tknHlK/2RpUU/FAkEA+H/jmArywrHd
7GE9sSH59nFuVDN1I2ev9N/GCZUEKkfWNcpKca7u3ZtznWjlzOH/lEZj7S53o4Cn
o8802sLpNwJBAPu1ePoFaVzlS8K5ZqnLMKHitQq5zf+dx725sdGP3ujqsBmC3GvU
/B0srv/y1QcgkmXH83nhAAr5ouU8cVwUaEMCQQDsKzdVj8oqCTsd5UjpaJ4UfrdZ
ZNJGBMwwkN+4yZv8xuspkgjwRE4V5SCbEE+eSYdKFx/vEYN4q9QnFc1ov2c/AkEA
m4uKRd7gPHLJ/Xw60ARrei6xt/4YKbUkAL9m/lyM4Jxe1HUEp3JHlfPN9Qbn0+6x
6UnF0PRU5XNc0ZInb0KBJwJALfS8ZWWTt/g8AVidtFsm/T+ctssB/4yVZRxbCjQJ
MYyKCg1ETN3r9l8f4tfgZAfxc4Ygdq6BKT7hvC4sWYd2Pg==
-----END RSA PRIVATE KEY-----"#;
    let public_key_xml = r#"-----BEGIN RSA PUBLIC KEY-----
MIGJAoGBAPRVjACiEBA3Y/0gEjjMynU9KDtiMDoQQs8FxvGLxsQ8AO/F/jWVbTO1
QLqtJbA+LHaJFeRTC5b7JiWEEiRbKKrGQQ8ylM3IbWZPNBr7a9RH+FFx7Ren2ke9
Nk0dPYaQQ1jg/zagE1wBc6xd5CspzO76fm23sk7kEkWwrAVsDWFlAgMBAAE=
-----END RSA PUBLIC KEY-----
"#;

    let signature = match sign_data(data, private_key_xml) {
        Ok(sig) => sig,
        Err(err) => panic!("Error signing data: {}", err),
    };

    let is_valid = match verify_signature(data, &signature, public_key_xml) {
        Ok(valid) => valid,
        Err(err) => panic!("Error verifying signature: {}", err),
    };

    println!("Signature verification: {}", if is_valid { "Valid" } else { "Invalid" });
}