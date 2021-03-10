use notes::auth::{Auth, AuthUser};
use notes::edge::Edge;
use notes::post::Post;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;

use super::NoteError;
use crate::DbConn;

#[get("/api/post/<id>/to")]
pub async fn get_to_list(conn: DbConn, id: u32) -> Json<NoteError<Vec<Edge>>> {
    Json(
        conn.run(move |conn| -> NoteError<Vec<Edge>> { Ok(Edge::get_to_list(conn, id)?) })
            .await,
    )
}

#[get("/api/post/<id>/from")]
pub async fn get_from_list(conn: DbConn, id: u32) -> Json<NoteError<Vec<Edge>>> {
    Json(
        conn.run(move |conn| -> NoteError<Vec<Edge>> { Ok(Edge::get_from_list(conn, id)?) })
            .await,
    )
}

#[derive(Deserialize)]
pub struct UpdateEdgeData {
    auth: Auth,
    data: Vec<u32>,
}
#[post("/api/post/<id>/to", data = "<data>")]
pub async fn update_to_list(
    conn: DbConn,
    id: u32,
    data: Json<UpdateEdgeData>,
) -> Json<NoteError<()>> {
    let data = data.into_inner();
    let auth = data.auth;
    let data = data.data;
    Json(
        conn.run(move |conn| -> NoteError<()> {
            let mut post_list: Vec<Post> = vec![];
            for id in data {
                post_list.push(Post::from_id(conn, id)?);
            }
            let authuser = AuthUser::try_from((auth, &*conn))?;
            Ok(Edge::update_to_list(
                conn,
                &authuser,
                id,
                post_list.iter().collect(),
            )?)
        })
        .await,
    )
}

#[post("/api/post/<id>/from", data = "<data>")]
pub async fn update_from_list(
    conn: DbConn,
    id: u32,
    data: Json<UpdateEdgeData>,
) -> Json<NoteError<()>> {
    let data = data.into_inner();
    let auth = data.auth;
    let data = data.data;
    Json(
        conn.run(move |conn| -> NoteError<()> {
            let mut post_list: Vec<Post> = vec![];
            for id in data {
                post_list.push(Post::from_id(conn, id)?);
            }
            let authuser = AuthUser::try_from((auth, &*conn))?;
            Ok(Edge::update_from_list(
                conn,
                &authuser,
                id,
                post_list.iter().collect(),
            )?)
        })
        .await,
    )
}
