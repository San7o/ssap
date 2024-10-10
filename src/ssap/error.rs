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

use std::fmt::{Display, Error, Formatter};

/// Error types for the SSAP library
#[derive(Debug)]
pub enum SsapError {
    InvalidKey,
    InvalidCiphertext,
    InvalidWrite,
    InvalidPath,
    InvalidPasswordName,
    InvalidEncryptionName,
    InvalidCommand,
    InvalidPasswordLength,
    InvalidPassword,
    PasswordMismatch,
    PasswordAlreadyRegistered,
    MissingPasswordName,
    MissingPasswordLength,
    MissingPath,
    ErrorDecrypting,
    ErrorSavingClipboard,
    ErrorGeneratingPassword,
    ErrorGeneratingIV,
    PasswordNameNotFound,
}

impl Display for SsapError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            SsapError::InvalidKey => write!(f, "Invalid key"),
            SsapError::InvalidCiphertext => write!(f, "Invalid ciphertext"),
            SsapError::InvalidWrite => write!(f, "Invalid write"),
            SsapError::InvalidPath => write!(f, "Invalid path"),
            SsapError::InvalidPasswordName => {
                write!(f, "Invalid password name")
            }
            SsapError::InvalidEncryptionName => {
                write!(f, "Invalid encryption name")
            }
            SsapError::InvalidCommand => write!(f, "Invalid command"),
            SsapError::InvalidPasswordLength => {
                write!(f, "Invalid password length")
            }
            SsapError::InvalidPassword => write!(f, "Invalid password"),
            SsapError::PasswordMismatch => write!(f, "Password mismatch"),
            SsapError::PasswordAlreadyRegistered => {
                write!(f, "Password already registered")
            }
            SsapError::MissingPasswordName => {
                write!(f, "Missing password name")
            }
            SsapError::MissingPasswordLength => {
                write!(f, "Missing password length")
            }
            SsapError::MissingPath => write!(f, "Missing path"),
            SsapError::ErrorDecrypting => write!(f, "Error decrypting"),
            SsapError::ErrorSavingClipboard => {
                write!(f, "Error saving clipboard")
            }
            SsapError::ErrorGeneratingPassword => {
                write!(f, "Error generating password")
            }
            SsapError::ErrorGeneratingIV => write!(f, "Error generating IV"),
            SsapError::PasswordNameNotFound => {
                write!(f, "Password name not found")
            }
        }
    }
}
