use hell_orm::prelude::*;


#[derive(Model)]
#[table_name = "users"]
pub struct User {
    #[primary_key]
    id: usize,

    #[unique]
    name: String,
}

#[derive(Model)]
#[table_name = "posts"]
pub struct Post {
    #[primary_key]
    id: usize,

    name: String,
    content: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::open::<schema![User, Post]>("local.db")?;

    Ok(())
}


