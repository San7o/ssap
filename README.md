# ssap - simply secure all passwords

ssap is a secure and **blazingly fast** password manager tool written in **Rust**.
You can safely store, organize, generate and retrieve passwords from your
local hard disk **blazingly quickly**. All the passwords will be stored in an encrypted form.

> Note: the project is currently in early developement and is changing **blazingly rapidly**

saap is:

- **secure**: written in Rust, but using battle tested encryption libraries (openssl) written in **C**
- **blazingly fast**: did I tell you that this program is **blazingly fast**? It is because It's written in Rust!
- **kiss**: keep It simple, stupendo! ssap follows the unix principle of doing one thing and doing it right.

## Usage
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
    - encrypt / decrypt
    - store / read from disk
    - generate passwords
- v2.0
    - multiple encryption algorithms
    - decrypt from url
    - 2fa?
    - **blazing speed**
