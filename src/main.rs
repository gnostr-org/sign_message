///https://docs.rs/secp256k1/latest/secp256k1/struct.Keypair.html#impl-Keypair
fn main() {
    use secp256k1::{Keypair, Secp256k1, SecretKey};
    use std::str::FromStr;
    let secp = Secp256k1::new();

    let private_key_arg = std::env::args()
        .nth(1)
        .expect("Missing private key argument");
    let key = SecretKey::from_str(&private_key_arg).unwrap();

    #[cfg(debug_assertions)]
    //sign_message 0000000000000000000000000000000000000000000000000000000000000001
    assert_eq!(
        "0000000000000000000000000000000000000000000000000000000000000001",
        format!("{}", key.display_secret())
    );

    let key_pair = Keypair::from_secret_key(&secp, &key);

    #[cfg(debug_assertions)]
    assert_eq!(
        "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        format!("{}", key_pair.public_key())
    );

    println!("secret_key={:}", &key_pair.display_secret());
    println!("public_key={:}", &key_pair.public_key());

    use secp256k1::hashes::sha256;
    use secp256k1::Message;

    #[cfg(debug_assertions)]
    let empty_str: &'static str = "";
    #[cfg(debug_assertions)]
    println!("empty_str={}", empty_str);

    #[cfg(debug_assertions)]
    let message_hash = Message::from_hashed_data::<sha256::Hash>(empty_str.as_bytes());
    #[cfg(debug_assertions)]
    assert_eq!(
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        format!("{}", message_hash)
        );

    //sign_message 0000000000000000000000000000000000000000000000000000000000000005 ""
    let message_str = std::env::args()
        .nth(2)
        .expect("Missing message string");
    let message_hash = Message::from_hashed_data::<sha256::Hash>(message_str.as_bytes());

    println!("message_hash={}", message_hash);

    let sig = secp.sign_ecdsa(&message_hash, &key);
    println!("sig={}", sig);

    assert!(secp
        .verify_ecdsa(&message_hash, &sig, &key_pair.public_key())
        .is_ok());
}
