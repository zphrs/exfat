use std::{fmt::Display, ops::Deref};

pub struct Index(u8);

#[derive(Debug)]
pub enum Error {
    TooBig(u8),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TooBig(i) => {
                write!(
                    f,
                    "Index of {} is too big. Extended Boot Sector index should be less than 8.",
                    i
                )
            }
        }
    }
}

impl Index {
    pub fn new(index: u8) -> Result<Self, Error> {
        if index > 7 {
            Err(Error::TooBig(index))
        } else {
            Ok(Self(index))
        }
    }
}

impl TryFrom<u8> for Index {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Deref for Index {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
