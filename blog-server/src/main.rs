#[macro_use]
extern crate rocket;
use rocket::{serde::json::{self, serde_json::json, Value}, figment::value::Value};
use rocket_db_pools::sqlx::{
    self,
    postgres::{PgRow},
    Row,
};
use rocket_db_pools::{Connection, Database};
mod data_types;
use data_types::{FailureData, User};

#[derive(Database)]
#[database("blog")]
struct Blog(sqlx::PgPool);

#[get("/")]
async fn index(mut db: Connection<Blog>) -> Result<Value, &'static str> {
    const SQL: &str = "SELECT a.id, a.title, a.content FROM public.article AS a LEFT JOIN (SELECT b.name, b.desc FROM public.user AS b) ON a.author_id = b.id";
    let _result: Result<Vec<PgRow>, sqlx::Error> = sqlx::query(&SQL).fetch_all(&mut *db).await;

    match result {
        Ok(v) => {

            Ok(json!({

            }))
        }
        Err(err) => {
            println!("query all article error, {}", err);
            Err("query all article error")
        }
    }
}

#[catch(500)]
fn error_catcher() -> Option<Value> {
    Some(json!({
            "success": false,
            "msg": String::from("get all article fail!"),
            "data": FailureData(()),
    }))
}

#[get("/article?<id>")]
async fn query_one(mut db: Connection<Blog>, id: &str) -> () {
    let sql_query_str = format!("SELECT id, name, desc, phone, reg_time, email FROM public.user WHERE id={id}");
    let rt = sqlx::query(&sql_query_str).fetch_all(&mut *db).await;
    match rt {
        Ok(res) => {
            let e: Vec<_> = res
                .iter()
                .map(|row: &PgRow| {
                    let user = User {
                        id: row.get(0),
                        name: row.get(1),
                        desc: row.get(2),
                        reg_time: row.get(4),
                        email: row.get(5),
                        phone: row.get(3),
                    };

                    json::serde_json::to_string(&user).unwrap()
                })
                .collect();
            dbg!(e);
        }
        Err(err) => {
            panic!("{}", err);
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Blog::init())
        .mount("/", routes![query_one])
}
