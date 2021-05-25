use notes_lib::auth::{Auth, AuthUser};
use notes_lib::post::Post;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;

use super::NoteError;
use crate::DbConn;

#[derive(Deserialize)]
struct NewPost {
    title: String,
    markdown: String,
}
#[derive(Deserialize)]
pub struct NewPostData {
    auth: Auth,
    data: NewPost,
}
#[put("/api/post", data = "<data>")]
pub async fn new_post(conn: DbConn, data: Json<NewPostData>) -> Json<NoteError<u32>> {
    Json(
        conn.run(move |conn| -> NoteError<u32> {
            use notes_lib::auth::AuthInsert;
            let data = data.into_inner();
            let auth = data.auth;
            let post = Post::from(data.data);
            let authuser = AuthUser::try_from((auth, &*conn))?;

            Ok(post.insert(conn, &authuser)?)
        })
        .await,
    )
}

#[post("/api/post/<id>", data = "<data>")]
pub async fn update_post(conn: DbConn, data: Json<NewPostData>, id: u32) -> Json<NoteError<()>> {
    Json(
        conn.run(move |conn| -> NoteError<()> {
            use notes_lib::auth::AuthUpdate;
            let data = data.into_inner();
            let auth = data.auth;
            let post = data.data;
            let post = Post::new(Some(id), post.title, Some(post.markdown));
            let authuser = AuthUser::try_from((auth, &*conn))?;

            Ok(post.update(conn, &authuser)?)
        })
        .await,
    )
}

#[derive(Deserialize, Serialize)]
pub struct ReturnPost {
    id: u32,
    markdown: String,
    title: String,
}
#[get("/api/post/<id>")]
pub async fn return_post(conn: DbConn, id: u32) -> Json<NoteError<ReturnPost>> {
    Json(
        conn.run(move |conn| -> NoteError<ReturnPost> {
            Ok(ReturnPost::from(&Post::from_id(conn, id)?))
        })
        .await,
    )
}

#[derive(Deserialize)]
pub struct DeletePostData {
    auth: Auth,
}
#[delete("/api/post/<id>", data = "<data>")]
pub async fn delete_post(conn: DbConn, id: u32, data: Json<DeletePostData>) -> Json<NoteError<()>> {
    Json(
        conn.run(move |conn| -> NoteError<()> {
            use notes_lib::auth::AuthDelete;

            let data = data.into_inner();
            let auth = data.auth;
            let authuser = AuthUser::try_from((auth, &*conn))?;

            Ok(Post::from_id(conn, id)?.delete(conn, &authuser)?)
        })
        .await,
    )
}

impl From<NewPost> for Post {
    fn from(post: NewPost) -> Post {
        Post::new(None, post.title, Some(post.markdown))
    }
}

impl From<&Post> for ReturnPost {
    fn from(post: &Post) -> ReturnPost {
        ReturnPost {
            id: post.get_id(),
            markdown: post.get_markdown().to_string(),
            title: post.get_title().to_string(),
        }
    }
}
