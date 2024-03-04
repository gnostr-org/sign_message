## [sign_message](https://github.com/gnostr-org/sign_message)

##### rust: sign a message using secp256k1

<hr><br>

#### install cargo, rustup, make

##### macos:

```shell
brew install cargo
brew install rustup
brew install make

```

##### linux:

```shell
apt install cargo
apt install rustup
apt install make

```

#### git:

```shell
git clone https://github.com/gnostr-org/sign_message.git

```

OR

```shell
git clone git@github.com:gnostr-org/sign_message.git

```

#### make:

```
make cargo-i
```

#### sign_message

```shell
sign_message
```

```
    Usage:

	sign_message <private_key> - print <public_key>

	sign_message <private_key> <string> - print signature of <string>

    Example:

	sign_message 0000000000000000000000000000000000000000000000000000000000000001 ""
    Expected:

```
```json
    [{"pubkey_xot_0":"60746bfca236edd5a8ca0f2db021f51180155ae714ffd98b8e83ff6aefb0b502"},{"pubkey_xot_1":"Even"},{"x_public_key":"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"},{"message_str":""},{"message_hash":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"},{"sig":"3044022077c8d336572f6f466055b5f70f433851f8f535f6c4fc71133a6cfd71079d03b702200ed9f5eb8aa5b266abac35d416c3207e7a538bf5f37649727d7a9823b1069577"}]
```


### Try:

```shell
cargo install gnostr-xq
```

```shell
sign_message 0000000000000000000000000000000000000000000000000000000000000001 "" | gnostr-xq

```

### Output:

```json 
[
  {
    "pubkey_xot_0": "a4e40162a71b581213b36ed1fe4e94b34279d5f1eb308e8226d4790c97dca7e0"
  },
  {
    "pubkey_xot_1": "Odd"
  },
  {
    "x_public_key": "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"
  },
  {
    "message_str": ""
  },
  {
    "message_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
  },
  {
    "sig": "3044022077c8d336572f6f466055b5f70f433851f8f535f6c4fc71133a6cfd71079d03b702200ed9f5eb8aa5b266abac35d416c3207e7a538bf5f37649727d7a9823b1069577"
  }
]


```

<hr><br>
FOR DEMONSTRATION PURPOSES ONLY!!!
<hr><br>
