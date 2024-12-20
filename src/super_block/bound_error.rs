extern crate alloc;
use core::fmt::Display;

use alloc::{format, string::String};

#[derive(Debug)]
pub enum BoundError {
    TooSmall {
        field: String,
        value: u64,
        should_be: u64,
    },
    TooBig {
        field: String,
        value: u64,
        should_be: u64,
    },
}

impl BoundError {
    pub fn too_small(
        field: impl Into<String>,
        value: impl Into<u64>,
        should_be: impl Into<u64>,
    ) -> Self {
        Self::TooSmall {
            field: field.into(),
            value: value.into(),
            should_be: should_be.into(),
        }
    }

    pub fn too_big(
        field: impl Into<String>,
        value: impl Into<u64>,
        should_be: impl Into<u64>,
    ) -> Self {
        Self::TooBig {
            field: field.into(),
            value: value.into(),
            should_be: should_be.into(),
        }
    }
}

impl Display for BoundError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            BoundError::TooSmall {
                field,
                value,
                should_be,
            } => {
                write!(f, "Field {} = {} is not >= {}", field, value, should_be)
            }
            BoundError::TooBig {
                field,
                value,
                should_be,
            } => {
                write!(f, "Field {} = {} is not <= {}", field, value, should_be)
            }
        }
    }
}

impl core::error::Error for BoundError {}
