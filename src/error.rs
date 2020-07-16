use std::fmt;

#[derive(Debug)]
pub enum Error {
    WebviewNull,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::WebviewNull => "Webview instance is null. Probably already dropped.".fmt(f),
        }
    }
}

impl std::error::Error for Error {}