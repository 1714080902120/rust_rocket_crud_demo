pub fn get_article(all: bool, art_id: &str, user_id: &str) -> String {
    let mut sql = String::from("SELECT a.id, a.title, a.content, a.author_id, b.name, b.desc FROM public.article AS a LEFT JOIN public.user AS b ON a.author_id = b.id");
    if !all {
        sql += match (art_id.is_empty(), user_id.is_empty()) {
            (true, true) => {
                format!(" WHERE id = {art_id} And author_id = {user_id}")
            }
            (false, true) => {
                format!(" WHERE author_id = {user_id}")
            }
            (true, false) => {
                format!(" WHERE id = {art_id}")
            }
            _ => String::from(""),
        }
        .as_str()
    }
    sql
}

// pub sql_user_by_id(id: &str) -> String {
//   format!("SELECT name, desc, reg_time FROM")
// }
