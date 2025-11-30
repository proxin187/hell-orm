use hell_orm::model::Model;
use hell_orm::prelude::*;


#[derive(Model)]
#[table_name = "users"]
pub struct User {
    #[primary_key]
    id: usize,

    #[unique]
    name: String,
}

impl Model for User {
    const COLUMNS: &'static [(&'static str, &'static str)] = &[("id", "INTEGER PRIMARY KEY"), ("name", "TEXT NOT NULL")];
    const NAME: &'static str = "users";
}

pub struct Post {
    id: usize,
    name: String,
    content: String,
}

impl Model for Post {
    const COLUMNS: &'static [(&'static str, &'static str)] = &[("id", "INTEGER PRIMARY KEY"), ("name", "TEXT NOT NULL"), ("content", "TEXT")];
    const NAME: &'static str = "posts";
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::open::<schema![User, Post]>("local.db")?;

    Ok(())
}


