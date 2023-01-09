use crate::model::{CreateUser,User};
use lazy_static::lazy_static;
use std::sync::{Arc,Mutex};

lazy_static! {
    static ref USERS: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(Vec::new()));
}

pub fn new_user(cu:CreateUser) -> u64 {
    let u_list = USERS.clone();
    let id:u64 = (u_list.lock().unwrap().len() + 1) as u64;
    let u = User {
        id,
        username: cu.username,
    };
    u_list.lock().unwrap().push(u);
    id
}

pub fn list_user() -> Vec<User> {
    USERS.clone().lock().unwrap().to_vec()
}