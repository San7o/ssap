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

OPTIONS:
    new               Create a new password
    get               Get an existing password
    delete            Delete an existing password
    list              List all registered passwords

INPUT:
    The name of the password to create or get. The password
    itself will be prompted for.

FLAGS:
    -h, --help         Prints help information
    -c, --clipboard    Copy the generated password to clipboard
    -s, --silent       Do not print the generated password
    -p, --path <path>  Specify the path to the password file
    -e, --encryption <encryption> Specify the encryption algorithm
                       Supported algorithms: aes_128_cbc, aes_256_cbc
    -l, --length <length> Specify the length of the generated password
                          default: 30

EXAMPLES:
    ssap new my_password
    ssap get my_password
    ssap generate my_password
```

ssap is a secure and **blazingly fast** password manager tool written in **Rust**.
You can safely store, organize, generate and retrieve passwords from your
local disk **blazingly quickly**. All the passwords will be stored in an encrypted form.

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

## Usage
Ssap is useful to generate safely store random password in a central location.

You can generate a new password using the `new` option:
```bash
ssap new linkedin
```
This will generate a new random password. You can specify the length of the password
via the `-l` flag. Ssap will prompt you to insert a password to encrypt the random
one, and then save it in `.ssap.enc`. You can specify the file path with the `--path` flag.
```bash
ssap new linkedin -l 20 --path ~/.my_secure_store
> Creating new password with name: linkedin
> Generated Password: NTp9g5yWgr%TU$f@!hwh
> Enter vault password:
> Re-enter vault password:
> Saving password to file in path: /home/lanto/.my_secure_store
> Password created successfully
```
You can list the registered passwords with the `list` option:
```bash
ssap list --path ~/.my_secure_store
> List of registered passwords:
>  - linkedin
>  - youtube
```
Get the password with the `get` option:
```bash
ssap get linkedin --path ~/.my_secure_store
> Enter vault password:
> Re-enter vault password:
> Decrypted Password: NTp9g5yWgr%TU$f@!hwh
```
And delete the password with `delete`:
```bash
ssap delete linkedin --path ~/.my_secure_store
> Password delete successfully
```

## Developmement
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
