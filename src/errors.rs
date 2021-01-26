use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u32)]
pub enum PartsErrorCode {
    LockError = 1,
    MissingPartError = 2,
    CreatePartError = 3,
    RequestError = 4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartsError {
    code: PartsErrorCode,
    description: String,
}

impl PartsError {
    pub fn new(code: PartsErrorCode, description: String) -> PartsError {
        PartsError { code, description }
    }
}
