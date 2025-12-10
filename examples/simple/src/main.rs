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

// TODO: rough sketch of how our proc macro generated code is to look like with the new builder system
pub trait __ContainsPostName {}

impl<T> __ContainsPostName for __HasPostName<T> {}
impl<T: __ContainsPostName> __ContainsPostName for __HasPostContent<T> {}

pub struct __HasPostName<T>(::std::marker::PhantomData<T>);
pub struct __HasPostContent<T>(::std::marker::PhantomData<T>);

pub struct __PostBuilder<T> {
    token: T,
    id: Option<usize>,
    name: Option<String>,
    content: Option<String>,
}

impl<T> __PostBuilder<T> {
    pub fn id(self, id: usize) -> __PostBuilder<T> {
        __PostBuilder {
            token: self.token,
            id: Some(id),
            name: self.name,
            content: self.content,
        }
    }

    pub fn name(self, name: String) -> __PostBuilder<__HasPostName<T>> {
        __PostBuilder {
            token: __HasPostName(::std::marker::PhantomData),
            id: self.id,
            name: Some(name),
            content: self.content,
        }
    }
}

#[derive(Schema)]
#[models(User, Post)]
struct Schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db: Database<Schema> = Database::open("local.db")?;

    db.insert(User {
        id: 67,
        name: String::from("test"),
    })?;

    db.insert(Post {
        id: 67,
        name: String::from("test"),
        content: String::from("test"),
    })?;

    Ok(())
}


