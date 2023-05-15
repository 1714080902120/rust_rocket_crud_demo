use rocket_db_pools::{
    sqlx::{PgPool},
    Database,
    Initializer
};

#[derive(Database)]
#[database("blog")]
pub struct Blog(PgPool);

pub fn init_db_blog() -> Initializer<Blog> {
    Blog::init()
}