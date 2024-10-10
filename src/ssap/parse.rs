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

use crate::ssap::error::SsapError;
use crate::ssap::ssap::{Encryption, Ssap};
use std::env::Args;
use std::path::Path;

/// Parse the command line arguments into a Ssap struct
/// # Arguments
/// * `args` - The command line arguments
/// # Returns
/// * A Ssap struct with the parsed arguments
/// * An error if the arguments are invalid
///
pub fn parse(args: Args) -> Result<Ssap, SsapError> {
    let mut ssap = Ssap::default();
    let mut args = args.into_iter().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" | "help" => {
                ssap.show_help = true;
            }
            "get" => {
                ssap.get_passwd = true;
            }
            "new" => {
                ssap.create_new = true;
            }
            "delete" => {
                ssap.delete_passwd = true;
            }
            "list" => {
                ssap.list = true;
            }
            "-c" | "--clipboard" => {
                ssap.copy_to_clipboard = true;
            }
            "-s" | "--silent" => {
                ssap.silent = true;
            }
            "-l" | "--length" => {
                if let Some(length) = args.next() {
                    if let Ok(length) = length.parse::<usize>() {
                        ssap.password_len = length;
                    } else {
                        return Err(SsapError::InvalidPasswordLength);
                    }
                } else {
                    return Err(SsapError::MissingPasswordLength);
                }
            }
            "-p" | "--path" => {
                if let Some(path) = args.next() {
                    ssap.path = Path::new(&path.clone()).into();
                } else {
                    return Err(SsapError::MissingPath);
                }
            }
            "-e" | "--encryption" => {
                if let Some(encryption) = args.next() {
                    match encryption.as_str() {
                        "aes_128_cbc" => {
                            ssap.encryption = Encryption::Aes_128_cbc;
                        }
                        "aes_256_ecb" => {
                            ssap.encryption = Encryption::Aes_256_cbc;
                        }
                        _ => {
                            return Err(SsapError::InvalidEncryptionName);
                        }
                    }
                } else {
                    return Err(SsapError::InvalidEncryptionName);
                }
            }
            input => {
                ssap.input = Some(input.to_string());
            }
        }
    }

    return Ok(ssap);
}
