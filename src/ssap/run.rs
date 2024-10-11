/*
* MIT License
*
* Copyright (c) 2024 Giovanni Santini
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in
all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*
*/

use crate::ssap::crypto::{decrypt_password, encrypt_password};
use crate::ssap::error::SsapError;
use crate::ssap::ssap::Ssap;
use cli_clipboard;
use regex::Regex;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

/// Run the SSAP application
///
/// # Arguments
/// * `settings` - The SSAP settings
/// # Returns
/// * A SsapError if the application fails
///
pub fn run(mut settings: Ssap) -> Result<(), SsapError> {
    read_version(&settings.path, &mut settings.version)?;
    if settings.show_help {
        help();
        return Ok(());
    } else if settings.get_passwd {
        get_passwd(settings)?;
    } else if settings.create_new {
        create_new(settings)?;
    } else if settings.delete_passwd {
        delete(settings)?;
    } else if settings.list {
        list(settings)?;
    } else {
        help();
        return Err(SsapError::InvalidCommand);
    }

    Ok(())
}

fn read_version(path: &Box<Path>, version: &mut Box<String>) -> Result<(), SsapError> {
    let file = fs::read_to_string(path.clone());
    if file.is_err() {
        return Ok(());
    }

    let file = file.unwrap();
    let first_line = file.lines().next();
    if first_line.is_none() {
        return Ok(());
    }

    let first_line = first_line.unwrap();

    let re = Regex::new(r"[0-9]+\.[0-9]+\.[0-9]+");
    if re.is_err() {
        return Ok(());
    }
    let re = re.unwrap();
    if !re.is_match(first_line) {
        return Err(SsapError::InvalidVersion);
    }
    *version = Box::new(first_line.to_owned());
    Ok(())
}

fn create_new(settings: Ssap) -> Result<(), SsapError> {
    if settings.input.is_none() {
        return Err(SsapError::MissingPasswordName);
    }

    let input = settings.input.clone().unwrap();
    if check_password_exists(&input, settings.path.clone()) {
        return Err(SsapError::PasswordAlreadyRegistered);
    }

    println!("> Creating new password with name: {}", input);
    let new_passwd: String = generate_password(settings.password_len);
    println!("> Generated Password: {}", new_passwd);
    let key: String = read_passwd_pompt()?;
    let encrypted_passwd =
        encrypt_password(new_passwd.into(), key.into(), &settings.encryption)?;
    save_password(input, encrypted_passwd, &settings)?;
    println!("> Password created successfully");

    Ok(())
}

fn generate_password(size: usize) -> String {
    let charset = "abcdefghijklmnopqrstuvwxyz\
                   ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                   0123456789\
                   !@#$%^&()={}[]?";
    let mut password = String::new();
    for _ in 0..(size as i64) {
        let idx = rand::random::<usize>() % charset.len();
        password.push(charset.chars().nth(idx).unwrap());
    }
    password
}

fn read_passwd_pompt() -> Result<String, SsapError> {
    std::io::stdout().flush().unwrap();
    let passwd = rpassword::prompt_password("> Enter vault password: ");
    if passwd.is_err() {
        return Err(SsapError::InvalidPassword);
    }
    let passwd2 = rpassword::prompt_password("> Re-enter vault password: ");
    if passwd2.is_err() {
        return Err(SsapError::InvalidPassword);
    }
    let passwd = passwd.unwrap();
    if passwd != passwd2.unwrap() {
        return Err(SsapError::PasswordMismatch);
    }
    Ok(passwd)
}

fn save_password(
    name: String,
    passwd: Vec<u8>,
    settings: &Ssap,
) -> Result<(), SsapError> {
    println!(
        "> Saving password to file in path: {}",
        settings.path.display()
    );
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(settings.path.clone());
    if file.is_err() {
        return Err(SsapError::InvalidPath);
    }
    let mut file = file.unwrap();

    let file_metadata = fs::metadata(settings.path.clone());
    if file_metadata.is_err() {
        return Err(SsapError::InvalidPath);
    }
    let file_metadata = file_metadata.unwrap();
    let file_size = file_metadata.len();
    if file_size == 0 {
        if let Err(_e) = writeln!(&mut file, "{}", settings.version) {
            return Err(SsapError::InvalidWrite);
        }
    }

    if let Err(_e) = writeln!(&mut file, "{}: {}", name, hex::encode(passwd)) {
        return Err(SsapError::InvalidWrite);
    }
    Ok(())
}

fn check_password_exists(name: &String, path: Box<Path>) -> bool {
    let file = fs::read_to_string(path);
    if file.is_err() {
        return false;
    }

    for line in file.unwrap().lines().skip(1) {
        let mut parts = line.split(": ");
        if let Some(n) = parts.next() {
            if n == name {
                return true;
            }
        }
    }

    false
}

fn get_passwd(settings: Ssap) -> Result<(), SsapError> {
    if settings.input.is_none() {
        return Err(SsapError::MissingPasswordName);
    }

    let encrypted_password =
        read_password(settings.input.clone().unwrap(), settings.path.clone())?;
    let key = read_passwd_pompt()?;
    let decrypted_password =
        decrypt_password(encrypted_password, key.into(), &settings.encryption)?;
    if !settings.silent {
        println!("> Decrypted Password: {}", decrypted_password);
    }
    if settings.copy_to_clipboard {
        println!("> Copying password to clipboard");
        copy_to_clipboard(decrypted_password)?;
    }

    Ok(())
}

fn read_password(name: String, path: Box<Path>) -> Result<Vec<u8>, SsapError> {
    let file = fs::read_to_string(path);
    if file.is_err() {
        return Err(SsapError::InvalidPath);
    }
    for line in file.unwrap().lines().skip(1) {
        let mut parts = line.split(": ");
        if let Some(n) = parts.next() {
            if n == name {
                if let Some(p) = parts.next() {
                    let decoded = hex::decode(p);
                    if decoded.is_err() {
                        return Err(SsapError::InvalidCiphertext);
                    }
                    return Ok(decoded.unwrap());
                }
            }
        }
    }
    Err(SsapError::PasswordNameNotFound)
}

fn copy_to_clipboard(password: String) -> Result<(), SsapError> {
    if let Err(_e) = cli_clipboard::set_contents(password.clone()) {
        return Err(SsapError::ErrorSavingClipboard);
    }
    if cli_clipboard::get_contents().is_err() {
        return Err(SsapError::ErrorSavingClipboard);
    }
    if cli_clipboard::get_contents().unwrap() != password {
        return Err(SsapError::ErrorSavingClipboard);
    }
    Ok(())
}

fn delete(settings: Ssap) -> Result<(), SsapError> {
    if settings.input.is_none() {
        return Err(SsapError::MissingPasswordName);
    }

    let name = settings.input.clone().unwrap();
    let file = fs::read_to_string(settings.path.clone());
    if file.is_err() {
        return Err(SsapError::InvalidPath);
    }
    let binding = file.unwrap();
    let mut lines = binding.lines();
    let version = lines.next();
    if version.is_none() {
        return Err(SsapError::InvalidVersion);
    }
    let version = version.unwrap();
    let mut new_file = String::new();
    let mut found = false;
    while let Some(line) = lines.next() {
        let mut parts = line.split(": ");
        if let Some(n) = parts.next() {
            if n == name {
                found = true;
            } else {
                new_file.push_str(&format!("{}\n", line));
            }
        }
    }
    if !found {
        return Err(SsapError::PasswordNameNotFound);
    }
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(settings.path.clone());
    if file.is_err() {
        return Err(SsapError::InvalidPath);
    }
    let mut file = file.unwrap();
    if let Err(_e) = write!(&mut file, "{}\n{}", version, new_file) {
        return Err(SsapError::InvalidWrite);
    }

    println!("> Password deleted successfully");
    Ok(())
}

fn list(settings: Ssap) -> Result<(), SsapError> {
    let file = fs::read_to_string(settings.path.clone());
    if file.is_err() {
        return Err(SsapError::InvalidPath);
    }

    println!("> List of registered passwords:");
    for line in file.unwrap().lines().skip(1) {
        let mut parts = line.split(": ");
        if let Some(n) = parts.next() {
            println!(">  - {}", n);
        }
    }
    Ok(())
}

fn help() {
    println!(
        r"
 ________   ________  ________  ________   
|\   ____\ |\   ____\|\   __  \|\   __  \  
\ \  \___|_\ \  \___|\ \  \|\  \ \  \|\  \ 
 \ \_____  \\ \_____  \ \   __  \ \   ____\
  \|____|\  \\|____|\  \ \  \ \  \ \  \___|
    ____\_\  \ ____\_\  \ \__\ \__\ \__\   
   |\_________\\_________\|__|\|__|\|__|   
   \|_________\|_________|                 
    "
    );

    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("USAGE:");
    println!("    ssap [OPTIONS] [INPUT] [FLAGS]");
    println!();
    println!("OPTIONS:");
    println!("    new               Create a new password");
    println!("    get               Get an existing password");
    println!("    delete            Delete an existing password");
    println!("    list              List all registered passwords");
    println!();
    println!("INPUT:");
    println!("    The name of the password to create or get to query the"); 
    println!("    real encrypted password.");
    println!();
    println!("FLAGS:");
    println!("    -h, --help         Prints help information");
    println!("    -c, --clipboard    Copy the generated password to clipboard");
    println!("    -s, --silent       Do not print the generated password");
    println!("    -p, --path <path>  Specify the path to the password file");
    println!(
        "    -e, --encryption <encryption> Specify the encryption algorithm"
    );
    println!(
        "                       Supported algorithms: aes_128_cbc, aes_256_cbc"
    );
    println!("    -l, --length <length> Specify the length of the generated password");
    println!("                          default: 30");
    println!();
    println!("EXAMPLES:");
    println!("    ssap new my_password");
    println!("    ssap get my_password");
    println!("    ssap generate my_password");
    println!();
}
