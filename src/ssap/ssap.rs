/// Struct holding SSAP settings
#[derive(Debug, Clone, Default)]
pub struct Ssap {
    pub show_help: bool,
    pub create_new: bool,
    pub generate: bool,
    pub get_passwd: bool,
    pub copy_to_clipboard: bool,
    pub silent: bool,
    pub input: Option<String>,
}
