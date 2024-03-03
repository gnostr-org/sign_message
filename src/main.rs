fn main() {
    use secp256k1::{Keypair, Secp256k1, SecretKey};
    use std::str::FromStr;

    let secp = Secp256k1::new();
    let key =
        SecretKey::from_str("0000000000000000000000000000000000000000000000000000000000000001")
            .unwrap();
    let key_pair = Keypair::from_secret_key(&secp, &key);

    assert_eq!(
        "0000000000000000000000000000000000000000000000000000000000000001",
        format!("{}", key.display_secret())
    );
    assert_eq!(
        "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        format!("{}", key_pair.public_key())
    );

    println!("secret_key={:}", &key_pair.display_secret());
    println!("public_key={:}", &key_pair.public_key());

    use secp256k1::hashes::sha256;
    use secp256k1::Message;

    let str_hello_world: &'static str = "Hello World!";
    println!("str_hello_world={}", str_hello_world);

    let message_hash = Message::from_hashed_data::<sha256::Hash>(str_hello_world.as_bytes());
    println!("message_hash={}", message_hash);

    let sig = secp.sign_ecdsa(&message_hash, &key);
    println!("sig={}", sig);

    assert!(secp
        .verify_ecdsa(&message_hash, &sig, &key_pair.public_key())
        .is_ok());
}
