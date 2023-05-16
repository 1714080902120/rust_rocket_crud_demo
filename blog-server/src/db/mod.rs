use rocket_db_pools::{
    sqlx::{PgPool},
    Database,
    Initializer, Connection
};

#[derive(Database)]
#[database("blog")]
pub struct Blog(PgPool);

pub fn init_db_blog() -> Initializer<Blog> {
    Blog::init()
}

pub type BlogDBC = Connection<Blog>;
