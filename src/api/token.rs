use notes::auth::{Auth, AuthUser};
use rocket_contrib::json::Json;
use serde::Deserialize;

use std::convert::TryFrom;

use super::NoteError;
use crate::DbConn;

#[derive(Deserialize)]
pub struct User {
    nickname: String,
    password: String,
}

#[put("/api/token", data = "<user>")]
pub async fn get_token(conn: DbConn, user: Json<User>) -> Json<NoteError<(u32, String)>> {
    Json(
        conn.run(move |conn| -> NoteError<(u32, String)> {
            let user = AuthUser::try_from((
                Auth::Password((user.nickname.clone(), user.password.clone())),
                &*conn,
            ))?;
            Ok((user.get_id(), user.add_token(conn)?))
        })
        .await,
    )
}
