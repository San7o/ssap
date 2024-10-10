use crate::ssap::ssap::Ssap;
use std::path::Path;
use std::env::Args;

pub fn parse(args: Args) -> Ssap {
    let mut ssap = Ssap::default();
    let mut args = args.into_iter().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" | "help" => {
                ssap.show_help = true;
            }
            "new" => {
                ssap.create_new = true;
            }
            "get" => {
                ssap.get_passwd = true;
            }
            "generate" => {
                ssap.generate = true;
            }
            "-c" | "--clipboard" => {
                ssap.copy_to_clipboard = true;
            }
            "-s" | "--silent" => {
                ssap.silent = true;
            }
            "-p" | "--path" => {
                if let Some(path) = args.next() {
                    ssap.path = Path::new(&path.clone()).into();
                }
                else {
                    eprintln!("Error: path not provided");
                    std::process::exit(1);
                }
            }
            input => {
                ssap.input = Some(input.to_string());
            }
        }
    }

    return ssap;
}
