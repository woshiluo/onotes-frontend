use notes_lib::auth::{Auth, AuthUser};
use notes_lib::user::User;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;

use super::NoteError;
use crate::DbConn;

#[derive(Deserialize)]
pub struct NewUser {
    nickname: String,
    password: String,
    email: String,
}

#[put("/api/user", data = "<user>")]
pub async fn register_user(conn: DbConn, user: Json<NewUser>) -> Json<NoteError<u32>> {
    Json(
        conn.run(move |conn| -> NoteError<u32> { User::from(user.into_inner()).insert(conn) })
            .await,
    )
}

#[derive(Deserialize)]
pub struct UpdateUser {
    auth: Auth,
    data: NewUser,
}
#[post("/api/user/<id>", data = "<data>")]
pub async fn update_user(conn: DbConn, id: u32, data: Json<UpdateUser>) -> Json<NoteError<()>> {
    Json(
        conn.run(move |conn| -> NoteError<()> {
            use notes_lib::auth::AuthUpdate;
            let data = data.into_inner();
            let auth = data.auth;
            let user = data.data;
            let user = notes_lib::user::User::new(Some(id), user.nickname, user.password, user.email);

            let authuser = AuthUser::try_from((auth, &*conn))?;
            user.update(conn, &authuser)?;

            Ok(())
        })
        .await,
    )
}

#[derive(Deserialize, Serialize)]
pub struct ReturnUser {
    id: u32,
    nickname: String,
    email: String,
    admin: bool,
}
#[get("/api/user/<id>")]
pub async fn return_user(conn: DbConn, id: u32) -> Json<NoteError<ReturnUser>> {
    Json(
        conn.run(move |conn| -> NoteError<ReturnUser> {
            Ok(ReturnUser::from(User::from_user_id(id, conn)?))
        })
        .await,
    )
}

impl From<NewUser> for User {
    fn from(user: NewUser) -> User {
        notes_lib::user::User::new(None, user.nickname, user.password, user.email)
    }
}

impl From<User> for ReturnUser {
    fn from(user: User) -> ReturnUser {
        ReturnUser {
            id: user.get_id(),
            nickname: String::from(user.get_nickname()),
            email: String::from(user.get_email()),
            admin: user.is_admin(),
        }
    }
}
