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

pub struct __HasPostName<T>(::std::marker::PhantomData<T>);
pub struct __HasPostContent<T>(::std::marker::PhantomData<T>);

pub struct __PostBuilder<'a, T> {
    builder: ::hell_orm::model::insert::InsertBuilder<'a, T>,
    id: Option<usize>,
    name: Option<String>,
    content: Option<String>,
}

impl<'a> __PostBuilder<'a, __HasPostName<__HasPostContent<()>>> {
    fn finish(self) -> Result<(), ::hell_orm::error::Error> {
        Ok(())
    }
}

impl<'a, T> __PostBuilder<'a, T> {
    pub fn id(self, id: usize) -> __PostBuilder<'a, T> {
        __PostBuilder {
            builder: self.builder,
            id: Some(id),
            name: self.name,
            content: self.content,
        }
    }

    pub fn name(self, name: String) -> __PostBuilder<'a, __HasPostName<T>> {
        __PostBuilder {
            builder: ::hell_orm::model::insert::InsertBuilder::new(self.builder.connection, __HasPostName(::std::marker::PhantomData)),
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


