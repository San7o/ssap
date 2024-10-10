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

use std::path::Path;

// TODO: add more encryption algorithms
/// List of supported encryption algorithms
#[derive(Debug, Clone)]
pub enum Encryption {
    Aes_128_cbc,
    Aes_256_cbc,
}

/// Struct holding SSAP settings
#[derive(Debug, Clone)]
pub struct Ssap {
    pub show_help: bool,
    pub create_new: bool,
    pub get_passwd: bool,
    pub delete_passwd: bool,
    pub list: bool,
    pub copy_to_clipboard: bool,
    pub silent: bool,
    pub input: Option<String>,
    pub path: Box<Path>,
    pub encryption: Encryption,
}

impl Ssap {
    /// Create a new Ssap struct
    pub fn new() -> Self {
        Ssap {
            show_help: false,
            create_new: false,
            get_passwd: false,
            delete_passwd: false,
            list: false,
            copy_to_clipboard: false,
            silent: false,
            input: None,
            path: Path::new("./.ssap.enc").into(),
            encryption: Encryption::Aes_128_cbc,
        }
    }
}

impl Default for Ssap {
    fn default() -> Self {
        Ssap::new()
    }
}
