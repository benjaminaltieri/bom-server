use std::vec::Vec;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::{PartsError, PartsErrorCode};
use crate::parts_list::Part;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryResult {
    pub code: u32,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub result: Option<QueryResult>,
    pub data: Option<Vec<Part>>,
    pub error: Option<PartsError>,
}

impl Response {
    pub fn new() -> Response {
        Response {
            result: None,
            data: None,
            error: None,
        }
    }

    pub fn result(mut self, code: u32, description: &str) -> Response {
        self.result = Some(QueryResult {
            code,
            description: description.into(),
        });
        self
    }

    pub fn data(mut self, data: Vec<Part>) -> Response {
        self.data = Some(data);
        self
    }

    pub fn error(mut self, code: PartsErrorCode, description: &str) -> Response {
        self.error = Some(PartsError::new(code, description.into()));
        self
    }
}
