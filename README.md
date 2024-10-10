# ssap - simply secure all passwords

```
 ________   ________  ________  ________
|\   ____\ |\   ____\|\   __  \|\   __  \
\ \  \___|_\ \  \___|\ \  \|\  \ \  \|\  \
 \ \_____  \\ \_____  \ \   __  \ \   ____\
  \|____|\  \\|____|\  \ \  \ \  \ \  \___|
    ____\_\  \ ____\_\  \ \__\ \__\ \__\
   |\_________\\_________\|__|\|__|\|__|
   \|_________\|_________|


USAGE:
    ssap [OPTIONS] [INPUT] [FLAGS]

FLAGS:
    -h, --help         Prints help information
    -c, --clipboard    Copy the generated password to clipboard
    -s, --silent       Do not print the generated password
    -p, --path <path>  Specify the path to the password file
    -e, --encryption <encryption> Specify the encryption algorithm
                       Supported algorithms: aes_128_cbc, aes_256_cbc

OPTIONS:
    new               Create a new password
    get               Get an existing password
    delete            Delete an existing password
    list              List all registered passwords

INPUT:
    The name of the password to create or get. The password
    itself will be prompted for.

EXAMPLES:
    ssap new my_password
    ssap get my_password
    ssap generate my_password
```

ssap is a secure and **blazingly fast** password manager tool written in **Rust**.
You can safely store, organize, generate and retrieve passwords from your
local disk **blazingly quickly**. All the passwords will be stored in an encrypted form.

> Note: the project is currently in early developement and is changing **blazingly rapidly**

## saap is:

- **secure**: written in Rust, but using battle tested encryption libraries (openssl) written in **C**
- **blazingly fast**: did I tell you that this program is **blazingly fast**? It is because It's written in Rust!
- **kiss**: keep It simple, stupendo! ssap follows the unix principle of doing one thing and doing it right.

## Installation
Compile the program:
```bash
cargo build
```
Run ssap:
```bash
cargo run
```
Run tests:
```bash
cargo test
```
Build documentation:
```bash
cargo doc
```
Format code:
```bash
cargo fmt
```

## TODO
- v1.0
    - [x] symmetric encrypt / decrypt
    - [x] store / read / delete from disk
    - [x] multiple encryption algorithms
    - [ ] generate passwords and iv
    - [ ] proper testing
