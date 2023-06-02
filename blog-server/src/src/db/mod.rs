use rocket_db_pools::{sqlx::PgPool, Connection, Database, Initializer};

#[derive(Database)]
#[database("blog")]
pub struct Blog(PgPool);

pub fn init_db_blog() -> Initializer<Blog> {
    Blog::init()
}

pub type SqlxError = sqlx::Error;

pub type DbQueryResult<T> = Result<T, sqlx::Error>;

pub type BlogDBC = Connection<Blog>;

pub fn is_row_not_found(err: SqlxError) -> bool {
    match err {
        SqlxError::RowNotFound => true,
        _ => false,
    }
}
