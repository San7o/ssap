use crate::ssap::ssap::Ssap;
use std::process::exit;

pub fn run(settings: Ssap) {

    if settings.show_help {
        help();
        std::process::exit(0);
    }
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
    println!("    ssap [FLAGS] [OPTIONS] [INPUT]");
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
