use rocket_contrib::templates::Template;

use notes_lib::history::History;

use super::{PublicPost, TagPost};
use crate::DbConn;

#[derive(serde::Serialize)]
pub struct PublicHistory {
    pub id: u32,
    pub post_id: u32,
    pub html: String,
    pub time: String,
    pub markdown: String,
}

impl From<&notes_lib::history::History> for PublicHistory {
    fn from(history: &notes_lib::history::History) -> PublicHistory {
        PublicHistory {
            id: history.get_id(),
            post_id: history.get_post_id(),
            time: super::from_timestamp_to_rfc3339(Some(history)),
            html: super::parse_markdown(history.get_markdown()),
            markdown: history.get_markdown().to_string(),
        }
    }
}

#[derive(serde::Serialize)]
struct HistoryList {
    id: u32,
    title: String,
    list: Vec<PublicHistory>,
    drawer: Vec<TagPost>,
}

#[get("/post/<id>/history")]
pub async fn list_history(conn: DbConn, id: u32) -> Template {
    let post_list = conn
        .run(|conn| -> Vec<TagPost> {
            let list = notes_lib::edge::Edge::get_to_list(&*conn, 1).unwrap();
            let mut post_list: Vec<TagPost> = vec![];
            for edge in list {
                post_list.push(TagPost::from_id(&*conn, edge.get_to()));
            }

            post_list
        })
        .await;

    let title = conn
        .run(move |conn| PublicPost::from_id(&*conn, id))
        .await
        .title
        .clone();
    let list = conn
        .run(move |conn| -> Vec<History> { History::get_history(id, conn).unwrap() })
        .await
        .iter()
        .map(PublicHistory::from)
        .collect();
    let context = HistoryList {
        id,
        drawer: post_list,
        title,
        list,
    };

    Template::render("histories", &context)
}

#[derive(serde::Serialize)]
struct HistoryContext {
    id: u32,
    title: String,
    drawer: Vec<TagPost>,
    history: PublicHistory,
}
#[get("/post/<id>/history/<hid>")]
pub async fn show_history(conn: DbConn, id: u32, hid: u32) -> Template {
    let post_list = conn
        .run(|conn| -> Vec<TagPost> {
            let list = notes_lib::edge::Edge::get_to_list(&*conn, 1).unwrap();
            let mut post_list: Vec<TagPost> = vec![];
            for edge in list {
                post_list.push(TagPost::from_id(&*conn, edge.get_to()));
            }

            post_list
        })
        .await;

    let title = conn
        .run(move |conn| PublicPost::from_id(&*conn, id))
        .await
        .title
        .clone();
    let history = PublicHistory::from(
        &conn
            .run(move |conn| -> History { History::from_id(conn, hid).unwrap() })
            .await,
    );
    let context = HistoryContext {
        id,
        drawer: post_list,
        title,
        history,
    };

    Template::render("history", &context)
}
