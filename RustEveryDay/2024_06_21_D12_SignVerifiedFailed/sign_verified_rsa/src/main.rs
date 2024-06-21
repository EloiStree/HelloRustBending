// use ring::signature::{KeyPair, RsaKeyPair, RSA_PKCS1_SHA256};
// use ring::rand::SystemRandom;













// fn main() {


    
//     let pem_private_key= "-----BEGIN RSA PRIVATE KEY-----
// MIICXQIBAAKBgQCFD6ZNFwuuOY0U0ULsMtxEZtmQKPxNLv96CUfZ0uLhz30lMkg/
// MqjhpTonHt+aH09YqRYTghBQcfE9ZYmE3bz5Paxclp+PJjNtVEzBPARMcIqDCALf
// XwgvT0phuxacYm9ASKU/3YrfS3S3vEtgQvj0AwU9yaizIGIFFTDjRvgg8QIDAQAB
// AoGAWnqtxrXpB2uy93jkXOCozwpw3opAJevTZRRyezffd1Uz79slLXQxSl6kEH35
// 5j9sIQm5LUK//TO9qtYkkOiG2qVGdcYBJp68Kl5mwWxhOEY0L8q2S93VDvGoyiE8
// ++ZyvfJx6vuKpzmgtgW9fQbucSH7JFtCmYMbLBzShrAgyQECQQDd1Q6q/vIfoiSb
// l55Pr0VLO1WwI4UTyvuqAZDr8oMslCe7QF9WinBF6kfRkNxfmAaIvuHjYAQlan7E
// Fjn9NWepAkEAmY5O4gF2fexXHTUsASh+D6lDXNjEIXWb1gux5E8ba2cfan2vcAOc
// m9dqhqNt32gM7q5GoYQlibn4ePTF51kcCQJBALVOOZrQnJv2Le1tUlrWadA/Qp2f
// qlivAlnExenSYBvDS2XkCwf0RicegGxr9XG8EzsYqA7qnvjlvY4OjH7Dc2ECQB/a
// iHyTpxvqGgwiCpXW64eeKi4pfU4fkostc+KTknGOO9MgbXNEHImrfxqc7l7ou74L
// gvVgb1UJsgSNajxanNkCQQDdImYs8tLZlPs7L9WX2QRc0Tkl/l0o+WfOqf139BQy
// 1WmOLGPL4UZbSkgpegPQfhZFeTY18GXOluR7DG62QChL
// -----END RSA PRIVATE KEY-----";

    

//     // Generate a new RSA key pair0
//     let rng = SystemRandom::new();
//     let key_pair = RsaKeyPair::generate(&ring::rand::SystemRandom::new(), 1024).expect("Failed to generate key pair");

//     // Message to sign
//     let message = b"Hello, Rust!";

//     // Sign the message
//     let mut signature = vec![0; key_pair.public_modulus_len()];
//     key_pair.sign(&RSA_PKCS1_SHA256, &rng, message, &mut signature)
//         .expect("Failed to sign message");

//     // Verify the signature
//     let public_key = key_pair.public_key();
//     let is_valid = public_key
//         .verify(&RSA_PKCS1_SHA256, message, &signature)
//         .is_ok();
//     println!("Signature is valid: {}", is_valid);
// }







// // use rsa::pkcs1::DecodeRsaPrivateKey;


// // use rsa::{
// //     pkcs1v15,
// //     pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey},
// //     pss,
// //     traits::{PrivateKeyParts, PublicKeyParts},
// //     RsaPrivateKey, RsaPublicKey,
// // };
// // use sha2::Sha256;

// // #[cfg(feature = "pem")]
// // use rsa::pkcs8::LineEnding;

// // fn main() {
// //     println!("Hello, world!");

// //     let pem_private_key= "-----BEGIN RSA PRIVATE KEY-----
// // MIICXQIBAAKBgQCFD6ZNFwuuOY0U0ULsMtxEZtmQKPxNLv96CUfZ0uLhz30lMkg/
// // MqjhpTonHt+aH09YqRYTghBQcfE9ZYmE3bz5Paxclp+PJjNtVEzBPARMcIqDCALf
// // XwgvT0phuxacYm9ASKU/3YrfS3S3vEtgQvj0AwU9yaizIGIFFTDjRvgg8QIDAQAB
// // AoGAWnqtxrXpB2uy93jkXOCozwpw3opAJevTZRRyezffd1Uz79slLXQxSl6kEH35
// // 5j9sIQm5LUK//TO9qtYkkOiG2qVGdcYBJp68Kl5mwWxhOEY0L8q2S93VDvGoyiE8
// // ++ZyvfJx6vuKpzmgtgW9fQbucSH7JFtCmYMbLBzShrAgyQECQQDd1Q6q/vIfoiSb
// // l55Pr0VLO1WwI4UTyvuqAZDr8oMslCe7QF9WinBF6kfRkNxfmAaIvuHjYAQlan7E
// // Fjn9NWepAkEAmY5O4gF2fexXHTUsASh+D6lDXNjEIXWb1gux5E8ba2cfan2vcAOc
// // m9dqhqNt32gM7q5GoYQlibn4ePTF51kcCQJBALVOOZrQnJv2Le1tUlrWadA/Qp2f
// // qlivAlnExenSYBvDS2XkCwf0RicegGxr9XG8EzsYqA7qnvjlvY4OjH7Dc2ECQB/a
// // iHyTpxvqGgwiCpXW64eeKi4pfU4fkostc+KTknGOO9MgbXNEHImrfxqc7l7ou74L
// // gvVgb1UJsgSNajxanNkCQQDdImYs8tLZlPs7L9WX2QRc0Tkl/l0o+WfOqf139BQy
// // 1WmOLGPL4UZbSkgpegPQfhZFeTY18GXOluR7DG62QChL
// // -----END RSA PRIVATE KEY-----";

// //     let mut rng = rand::thread_rng();
 
// //     let private_key = RsaPrivateKey::from_pkcs1_pem(pem_private_key).expect("Failed to parse private key");
// //     //let private_key = RsaPrivateKey::from_pkcs8_pem(pem_private_key).unwrap();
// //     let public_key = RsaPublicKey::from(&private_key);

// //     // Message to sign
// //     let message = b"Hello, Rust!";

// //     // // Encrypt
// //     // let data = b"hello world";
// //     // let enc_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
// //     // assert_ne!(&data[..], &enc_data[..]);

// //     // // Decrypt
// //     // let dec_data = private_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
// //     // assert_eq!(&data[..], &dec_data[..]);

// //     // // Sign the message
// //     // let padding = SignatureScheme::new_pkcs1v15_signer();
// //     // let signature = private_key.sign(padding, &message).expect("Failed to sign message");

// //     // // Verify the signature
// //     // let is_valid = public_key.verify(padding, &message, &signature).is_ok();
// //     // println!("Signature is valid: {}", is_valid);

    
// //     // let mut signed_key = private_key.signer(pkcs1v15::Signature::new(Sha256::new()));

// //     // let bytes: &[u8] = b"rsa4096"; // HACK - the criterion is that the signature has leading zeros.
// //     // let signature = signing_key.sign(bytes);

// //     // let expected = "029E365B60971D5A499FF5E1C288B954D3A5DCF52482CEE46DB90DC860B725A8D6CA031146FA156E9F17579BE6122FFB11DAC35E59B2193D75F7B31CE1442DDE7F4FF7885AD5D6080266E9A33BB4CEC93FCC2B6B885457A0ABF19E2DAA00876F694B37F535F119925CCCF9A17B90AE6CF39F07D7FEFBEECDF1B344C14B728196DDD154230BADDEDA5A7EFF373F6CD3EF6D41789572A7A068E3A252D3B7D5D706C6170D8CFDB48C8E738A4B3BFEA3E15716805E376EBD99EA09C6E82F3CFA13CEB23CD289E8F95C27F489ADC05AAACE8A9276EE7CED3B7A5C7264F0D34FF18CEDC3E91D667FCF9992A8CFDE8562F65FDDE1E06595C27E0F82063839A358C927B2";
// //     // assert_eq!(format!("{}", signature), expected);
// //     // assert_eq!(format!("{:x}", signature), expected.to_lowercase());
// //     // assert_eq!(format!("{:X}", signature), expected);
// //     // assert_eq!(signature.to_string(), expected);


// // }



// // //https://docs.rs/ring/latest/ring/signature/index.html