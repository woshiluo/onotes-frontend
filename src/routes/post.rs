use rocket_contrib::templates::Template;

use super::{PublicPost, TagPost};
use crate::DbConn;

#[derive(serde::Serialize)]
struct PostContext {
    title: String,
    data: PublicPost,
    drawer: Vec<TagPost>,
}

#[get("/post/<id>")]
pub async fn view_post(conn: DbConn, id: u32) -> Template {
    let post_list = conn
        .run(|conn| -> Vec<TagPost> {
            let list = notes::edge::Edge::get_to_list(&*conn, 1).unwrap();
            let mut post_list: Vec<TagPost> = vec![];
            for edge in list {
                post_list.push(TagPost::from_id(&*conn, edge.get_to()));
            }

            post_list
        })
        .await;
    let current_post = conn.run(move |conn| PublicPost::from_id(&*conn, id)).await;

    let context = PostContext {
        title: current_post.title.clone(),
        data: current_post,
        drawer: post_list,
    };
    Template::render("post", &context)
}

#[derive(serde::Serialize)]
struct StringContext {
    title: String,
    markdown: String,
    from_json: String,
    to_json: String,
}
#[derive(serde::Serialize)]
struct EditContext {
    id: u32,
    title: String,
    drawer: Vec<TagPost>,
    data: StringContext,
}
#[get("/post/<id>/edit")]
pub async fn edit_post(conn: DbConn, id: u32) -> Template {
    let post_list = conn
        .run(|conn| -> Vec<TagPost> {
            let list = notes::edge::Edge::get_to_list(&*conn, 1).unwrap();
            let mut post_list: Vec<TagPost> = vec![];
            for edge in list {
                post_list.push(TagPost::from_id(&*conn, edge.get_to()));
            }

            post_list
        })
        .await;

    let current_post = conn
        .run(move |conn| {
            let post = notes::post::Post::from_id(&*conn, id).unwrap();
            StringContext {
                title: post.get_title().to_string(),
                markdown: post.get_markdown().to_string(),
                to_json: serde_json::to_string(
                    &notes::edge::Edge::get_to_list(&*conn, id)
                        .unwrap()
                        .iter()
                        .map(|x| x.get_to())
                        .collect::<Vec<u32>>(),
                )
                .unwrap(),
                from_json: serde_json::to_string(
                    &notes::edge::Edge::get_from_list(&*conn, id)
                        .unwrap()
                        .iter()
                        .map(|x| x.get_from())
                        .collect::<Vec<u32>>(),
                )
                .unwrap(),
            }
        })
        .await;

    // TODO: Add Title
    let context = EditContext {
        id,
        title: format!("Edit #{} {}", id, current_post.title),
        drawer: post_list,
        data: current_post,
    };
    Template::render("edit", &context)
}
