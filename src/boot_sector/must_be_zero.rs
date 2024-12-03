use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[derive(Serialize, Deserialize, Clone)]
pub struct MustBeZero(#[serde(with = "BigArray")] [u8; 53]);

impl Default for MustBeZero {
    fn default() -> Self {
        Self([0; 53])
    }
}
