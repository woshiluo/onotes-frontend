use rocket_contrib::templates::Template;

use super::PublicPost;
use crate::DbConn;

#[derive(serde::Serialize)]
struct IndexContext {
    title: String,
    main: PublicPost,
    data: Vec<PublicPost>,
}
#[get("/")]
pub async fn index(conn: DbConn) -> Template {
    let post_list = conn
        .run(|conn| -> Vec<PublicPost> {
            let list = notes::edge::Edge::get_to_list(&*conn, 1).unwrap();
            let mut post_list: Vec<PublicPost> = vec![];
            for edge in list {
                post_list.push(PublicPost::from_id(&*conn, edge.get_to()));
            }

            post_list
        })
        .await;

    let main_post = conn.run(|conn| PublicPost::from_id(&*conn, 1)).await;
    let context = IndexContext {
        title: "Index".to_string(),
        main: main_post,
        data: post_list,
    };
    Template::render("index", &context)
}
