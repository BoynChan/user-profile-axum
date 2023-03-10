use serde::{Deserialize, Serialize};
// the input to our `create_user` handler
#[derive(Deserialize, Clone)]
pub struct CreateUser {
    pub username: String,
}

// the output to our `create_user` handler
#[derive(Serialize, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
}
