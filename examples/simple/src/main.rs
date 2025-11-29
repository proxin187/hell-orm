use hell_orm::model::Model;
use hell_orm::{models, Database};


pub struct User {
    name: String,
}

impl Model for User {
    const CREATE: &'static str = "test";
}

pub struct Post {
    name: String,
}

impl Model for Post {
    const CREATE: &'static str = "test";
}

fn main() {
    let db = Database::open::<models![User, Post]>("local.db");
}


