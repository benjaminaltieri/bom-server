use std::vec::Vec;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct NewPart {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateChildren {
    pub children: Vec<Uuid>,
}
