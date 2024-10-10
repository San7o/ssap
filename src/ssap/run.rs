use crate::ssap::ssap::Ssap;

pub fn run(settings: Ssap) {

    if settings.show_help {
        help();
    }
}

fn help() {
    println!("ssap - Simple Secure Password Manager");
    println!();
    println!("USAGE:");
    println!("    ssap [FLAGS] [OPTIONS] [INPUT]");
    println!();
    println!("FLAGS:");
    println!("    -h, --help        Prints help information");
    println!("    -c, --clipboard    Copy the generated password to clipboard");
    println!("    -s, --silent       Do not print the generated password");
    println!();
    println!("OPTIONS:");
    println!("    new               Create a new password");
    println!("    get               Get an existing password");
    println!("    generate          Generate a new password");
    println!();
    println!("INPUT:");
    println!("    The name of the password to create or get");
    println!();
    println!("EXAMPLES:");
    println!("    ssap new my_password");
    println!("    ssap get my_password");
    println!("    ssap generate");
    println!();
}
