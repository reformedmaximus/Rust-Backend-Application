use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CustomDetail {
    pub name: String,
    pub email: String,
}
