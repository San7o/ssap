use crate::ssap::ssap::Ssap;
use crate::ssap::error::SsapError;
use std::process::exit;
use std::fs::{OpenOptions};
use std::fs;
use std::io::{Write};
use rpassword::{prompt_password};

pub fn run(settings: Ssap) -> Result<(), SsapError> {

    if settings.show_help {
        help();
        return Ok(());
    }
    else if settings.create_new {
        create_new(settings)?;
    }
    else if settings.get_passwd {
        get_passwd(settings)?;
    }
    else if settings.generate {
        generate(settings)?;
    }
    else if settings.delete_passwd {
        delete(settings)?;
    }

    Ok(())
}

fn create_new(settings: Ssap) -> Result<(), SsapError> {
    if settings.input.is_none() {
        return Err(SsapError::MissingPasswordName);
    }

    let input = settings.input.clone().unwrap();
    println!("Creating new password with name: {}", input);
    let passwd = read_passwd_pompt()?;

    // TODO: encrypt password

    save_password(input, passwd, settings)?;
    println!("Password created successfully");

    Ok(())
}

fn read_passwd_pompt() -> Result<String, SsapError> {
    std::io::stdout().flush().unwrap();
    let passwd = rpassword::prompt_password("Enter the new password: ").unwrap();
    let passwd2 = rpassword::prompt_password("Re-enter the new password: ").unwrap();
    if passwd != passwd2 {
        return Err(SsapError::PasswordMismatch);
    }
    Ok(passwd)
}

fn save_password(name: String, passwd: String, settings: Ssap) -> Result<(), SsapError> {
    println!("Saving password to file in path: {}", settings.path.display());
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(settings.path.clone());
    if file.is_err() {
        return Err(SsapError::InvalidPath);
    }
    if let Err(e) = writeln!(file.unwrap(), "{}: {}", name, passwd) {
        return Err(SsapError::InvalidWrite);
    }
    Ok(())
}

fn get_passwd(settings: Ssap) -> Result<(), SsapError> {
    if settings.input.is_none() {
        return Err(SsapError::MissingPasswordName);
    }

    let password = read_password(settings.input.clone().unwrap(), settings)?;

    // TODO: decrypt password

    Ok(())
}

fn read_password(name: String, settings: Ssap) -> Result<String, SsapError> {
    let path = settings.path;
    let file = fs::read_to_string(path);
    if file.is_err() {
        return Err(SsapError::InvalidPath);
    }
    for line in file.unwrap().lines() {
        let mut parts = line.split(": ");
        if let Some(n) = parts.next() {
            if n == name {
                if let Some(p) = parts.next() {
                    return Ok(p.to_string());
                }
            }
        }
    }
    Err(SsapError::InvalidPasswordName)
}


fn generate(settings: Ssap) -> Result<(), SsapError> {
    Ok(())
}

fn delete(settings: Ssap) -> Result<(), SsapError> {
    Ok(())
}

fn help() {
    println!(r"
 ________  ________  ________  ________   
|\   ____\|\   __  \|\   __  \|\   __  \  
\ \  \___|\ \  \|\  \ \  \|\  \ \  \|\  \ 
 \ \_____  \ \   __  \ \   __  \ \   ____\
  \|____|\  \ \  \ \  \ \  \ \  \ \  \___|
    ____\_\  \ \__\ \__\ \__\ \__\ \__\   
   |\_________\|__|\|__|\|__|\|__|\|__|   
   \|_________|                           
                                          
");
    println!("USAGE:");
    println!("    ssap [OPTIONS] [INPUT] [FLAGS]");
    println!();
    println!("FLAGS:");
    println!("    -h, --help         Prints help information");
    println!("    -c, --clipboard    Copy the generated password to clipboard");
    println!("    -s, --silent       Do not print the generated password");
    println!("    -p, --path <path>  Specify the path to the password file");
    println!("OPTIONS:");
    println!("    new               Create a new password");
    println!("    get               Get an existing password");
    println!("    generate          Generate a new password");
    println!("    delete            Delete an existing password");
    println!();
    println!("INPUT:");
    println!("    The name of the password to create or get. The password");
    println!("    itself will be prompted for.");
    println!();
    println!("EXAMPLES:");
    println!("    ssap new my_password");
    println!("    ssap get my_password");
    println!("    ssap generate my_password");
    println!();
}
