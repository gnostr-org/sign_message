use std::process;
fn print_version() {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    println!("v{}", VERSION.unwrap_or("unknown"));
    process::exit(0);
}
fn print_usage(code: i32) {
    println!("\n    Usage:\n");
    println!("\tsign_message <private_key> - print <public_key>\n");
    println!("\tsign_message <private_key> <string> - print signature of <string>");
    if code == 24 {
        println!("\t             ^private_key must be hexidecimal characters.");
    }
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

fn is_hex(text: &str) -> bool {
    use regex::Regex;
    let re = Regex::new(r"^[0-9a-fA-F]+$").unwrap();
    re.is_match(text)
}

//GLOBAL VARIABLES
//static GLOBAL_VAR_BOOL: &bool = &false;
//static GLOBAL_VAR_STRING: &str = "GLOBAL_VAR_STRING";
//END GLOBAL VARIABLES

fn main() -> Result<(), String> {
    let mut _verbose = false;
    use secp256k1::{Keypair, Scalar, Secp256k1, SecretKey, XOnlyPublicKey};
    use std::env;
    use std::str::FromStr;
    let secp = Secp256k1::new();
    let tweak = Scalar::random();

    let args: Vec<String> = env::args().collect();
    let _app_name = &args[0];

    //let _num_args = args.len();
    //#[cfg(debug_assertions)]
    //println!("_num_args - 1 = {}", _num_args - 1);
    if env::args().len() == 1 {
        print_usage(0);
    }

    if env::args().len() > 1 {
        //begin handle args
        //begin handle args
        //begin handle args

        let private_key_arg = std::env::args()
            .nth(1)
            .expect("Missing private key argument");

        if is_hex(&private_key_arg) {
        } else {
            print_usage(24);
        }
        //0000000000000000000000000000000000000000000000000000000000000000
        if &private_key_arg == "0000000000000000000000000000000000000000000000000000000000000000" {
            //TODO:use as special case
            print_usage(999);
        }
        if &private_key_arg == "-vv" || &private_key_arg == "--verbose" {
            _verbose = true;
            println!("verbose={}", _verbose)
        }
        if &private_key_arg == "-h" || &private_key_arg == "--help" {
            print_usage(0);
        }
        if &private_key_arg == "-v" || &private_key_arg == "--version" || &private_key_arg == "-V" {
            print_version();
        }

        if is_string_of_length_64(&private_key_arg) {
        } else {
            print_usage(64);
        }

        let private_key = SecretKey::from_str(&private_key_arg).unwrap();

        #[cfg(debug_assertions)]
        //sign_message 0000000000000000000000000000000000000000000000000000000000000001
        assert_eq!(
            "0000000000000000000000000000000000000000000000000000000000000001",
            format!("{}", private_key.display_secret())
        );
        //#[cfg(debug_assertions)]
        //println!("115:key.display_secret()={:?}", &key.display_secret());
        //#[cfg(debug_assertions)]
        println!(
            "118:{{\"private_key\": {:}}}",
            &private_key.display_secret()
        );

        let key_pair = Keypair::from_secret_key(&secp, &private_key);
        println!("123:{{\"public_key\": \"{}\"}}", &key_pair.public_key());
        let _pubkey_xo = XOnlyPublicKey::from_keypair(&key_pair);
        //#[cfg(debug_assertions)]
        //println!("122:pubkey_xo={:?}", _pubkey_xo);
        let (pubkey_xo, _parity) = key_pair.x_only_public_key();
        let pubkey_xot = pubkey_xo
            .add_tweak(&secp, &tweak)
            .expect("Improbable to fail with a randomly generated tweak");

        println!(
            "143:{{\"public_xot.0\": \"{:}\"}}",
            pubkey_xot.0.to_string()
        );
        println!("144:{{\"public_xot.1\": \"{:?}\"}}", pubkey_xot.1);

        //#[cfg(debug_assertions)]
        //println!("132:_tweaked={:?}", _tweaked);

        //#[cfg(debug_assertions)]
        //assert_eq!(
        //    "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        //    format!("{}", key_pair.public_key())
        //);

        //#[cfg(debug_assertions)]
        //println!(
        //    "145:key_pair.x_only_public_key()={:?}",
        //    key_pair.x_only_public_key()
        //);

        let (/*mut*/ x_public_key, _) = key_pair.x_only_public_key();
        //#[cfg(debug_assertions)]
        //println!("151:{{\"public_key\": \"{}\"}}", &key_pair.public_key());
        //println!("152:{{\"x_public_key\": \"{}\"}}", x_public_key);
        println!("152:{{\"x_public_key\": \"{:}\"}}", x_public_key);

        let x_original = x_public_key;
        let (tweaked, parity) = x_public_key
            .add_tweak(&secp, &tweak)
            .expect("Improbable to fail with a randomly generated tweak");
        assert!(x_original.tweak_add_check(&secp, &tweaked, parity, tweak));
        //#[cfg(debug_assertions)]
        //println!("160:{{\"x_original\": \"{}\"}}", x_original);
        //#[cfg(debug_assertions)]
        //println!("158:tweaked={:?}", tweaked);

        //#[cfg(debug_assertions)]
        //dbg!(args);
        if env::args().len() == 2 {
            #[cfg(debug_assertions)]
            //println!("168:{{\"private_key\": {:}}}", &key_pair.display_secret());
            println!("169:{{\"public_key\": \"{}\"}}", &key_pair.public_key());
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
        //println!("empty_str={}", empty_str);
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
        println!("208:message_hash={}", message_hash);

        let sig = secp.sign_ecdsa(&message_hash, &private_key);
        assert!(secp
            .verify_ecdsa(&message_hash, &sig, &key_pair.public_key())
            .is_ok());

        println!("{{\"sig\": \"{}\"}}", sig);
    } // end if env::args().len() > 1
    Ok(())
}
// This code defines a function to add two numbers
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This code is only compiled when running tests
#[cfg(test)]
mod tests {
    // Import the add function from the outer scope
    use super::*;

    // This function is marked as a test with the `#[test]` attribute
    #[test]
    fn test_add() {
        // This assertion checks if the sum of 1 and 2 is equal to 3
        assert_eq!(add(1, 2), 3);
    }
}
