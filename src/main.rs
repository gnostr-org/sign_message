use std::process;
fn print_version() {
const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
println!("v{}", VERSION.unwrap_or("unknown"));
}
fn print_usage(code: i32) {
    println!("\n    Usage:\n");
    println!("\tsign_message <private_key> - print <public_key>\n");
    println!("\tsign_message <private_key> <string> - print signature of <string>");
    if code == 64 {
        println!("\t             ^private_key must be 64 characters long.");
    }
    println!("\n    Example:\n");
    println!(
        "\tsign_message 0000000000000000000000000000000000000000000000000000000000000001 \"\""
    );
    if code == 999 {
        println!("\t                                       private_key must be greater than zero^");
    }
    println!("    Expected:\n");
    println!(
    "\t3044022077c8d336572f6f466055b5f70f433851f8f535f6c4fc71133a6cfd71079d03b702200ed9f5eb8aa5b266abac35d416c3207e7a538bf5f37649727d7a9823b1069577\n"
    );

    if code == 0 {
        process::exit(code);
    }
    if code == 64 {
        process::exit(code);
    }

    process::exit(0);
}

fn is_string_of_length_64(string: &str) -> bool {
    return string.len() == 64;
}

fn main() -> Result<(), String> {
    use secp256k1::{Keypair, Secp256k1, SecretKey};
    use std::env;
    use std::str::FromStr;
    let secp = Secp256k1::new();

    let args: Vec<String> = env::args().collect();

    let _num_args = args.len();
    #[cfg(debug_assertions)]
    println!("_num_args - 1 = {}", _num_args - 1);
    if env::args().len() == 1 {
        print_usage(0);
    }

    if env::args().len() > 1 {
        //begin handle args
        let private_key_arg = std::env::args()
            .nth(1)
            .expect("Missing private key argument");

        //0000000000000000000000000000000000000000000000000000000000000000
        if &private_key_arg == "0000000000000000000000000000000000000000000000000000000000000000" {
          //TODO:use as special case
            print_usage(999);
        }
        if &private_key_arg == "-h" || &private_key_arg == "--help" {
            print_usage(999);
        }
        if &private_key_arg == "-v" || &private_key_arg == "--version" || &private_key_arg == "-V"{
            print_version();
        }

        if is_string_of_length_64(&private_key_arg) {
        } else {
            print_usage(64);
        }

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

        #[cfg(debug_assertions)]
        dbg!(args);
        if env::args().len() == 2 {
            //println!("{{\"secret_key\": {}}}", &key_pair.display_secret());
            //println!("{{\"secret_key\": {:}}}", &key_pair.display_secret());
            //println!("{{\"secret_key\": {:?}}}", &key_pair.display_secret());

            println!("{{\"public_key\": \"{}\"}}", &key_pair.public_key());
            //println!("{{\"public_key\": {}}}", &key_pair.public_key());
            //println!("{{\"public_key\": {:}}}", &key_pair.public_key());
            //println!("{{\"public_key\": {:?}}}", &key_pair.public_key());
            process::exit(0);
        }

        #[cfg(debug_assertions)]
        println!("secret_key={:}", &key_pair.display_secret());
        #[cfg(debug_assertions)]
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
        let message_str = std::env::args().nth(2).expect("Missing message string");
        let message_hash = Message::from_hashed_data::<sha256::Hash>(message_str.as_bytes());

        #[cfg(debug_assertions)]
        println!("message_hash={}", message_hash);

        let sig = secp.sign_ecdsa(&message_hash, &key);
        assert!(secp
            .verify_ecdsa(&message_hash, &sig, &key_pair.public_key())
            .is_ok());

        println!("{}", sig);
    } // end if env::args().len() > 1
    Ok(())
}
