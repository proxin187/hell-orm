use hell_orm::prelude::*;


#[derive(Model)]
#[table_name = "users"]
pub struct User {
    #[primary_key]
    #[auto_increment]
    id: usize,

    #[unique]
    name: String,
}

#[derive(Model)]
#[table_name = "posts"]
pub struct Post {
    #[primary_key]
    #[auto_increment]
    id: usize,

    user: usize,
    content: String,
}

#[derive(Schema)]
#[models(User, Post)]
struct Schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db: Database<Schema> = Database::open("local.db")?;

    db.insert::<Post>()
        .user(1)
        .content(String::new())
        .finish()?;

    Ok(())
}


