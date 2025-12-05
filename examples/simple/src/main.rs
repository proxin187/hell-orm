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

#[derive(Schema)]
#[models(User, Post)]
struct Schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db: Database<Schema> = Database::open("local.db")?;

    db.insert(User {
        id: 67,
        name: String::from("test"),
    });

    db.insert(Post {
        id: 67,
        name: String::from("test"),
        content: String::from("test"),
    });

    Ok(())
}


