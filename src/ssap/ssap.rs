use std::path::Path;

/// Struct holding SSAP settings
#[derive(Debug)]
pub struct Ssap {
    pub show_help: bool,
    pub create_new: bool,
    pub generate: bool,
    pub get_passwd: bool,
    pub copy_to_clipboard: bool,
    pub silent: bool,
    pub input: Option<String>,
    pub path: Box<Path>,
}

impl Ssap {
    /// Create a new Ssap struct
    pub fn new() -> Self {
        Ssap {
            show_help: false,
            create_new: false,
            generate: false,
            get_passwd: false,
            copy_to_clipboard: false,
            silent: false,
            input: None,
            path: Path::new(".").into(),
        }
    }
}

impl Default for Ssap {
    fn default() -> Self {
        Ssap::new()
    }
}
