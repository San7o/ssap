use crate::ssap::ssap::Ssap;
use std::env::Args;

pub fn parse(args: Args) -> Ssap {
    let mut ssap = Ssap::default();
    for arg in args.skip(1) {
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
            input => {
                ssap.input = Some(input.to_string());
            }
        }
    }

    return ssap;
}
