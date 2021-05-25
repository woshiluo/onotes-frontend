use notes_lib::auth::{Auth, AuthUser};
use notes_lib::history::History;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;

use super::NoteError;
use crate::DbConn;

#[get("/api/post/<id>/history")]
pub async fn get_history(conn: DbConn, id: u32) -> Json<NoteError<Vec<History>>> {
    Json(
        conn.run(move |conn| -> NoteError<Vec<History>> { Ok(History::get_history(id, conn)?) })
            .await,
    )
}

#[derive(Deserialize)]
pub struct DeleteHistoryData {
    auth: Auth,
}
#[delete("/api/history/<id>", data = "<data>")]
pub async fn delete_history(
    conn: DbConn,
    id: u32,
    data: Json<DeleteHistoryData>,
) -> Json<NoteError<()>> {
    Json(
        conn.run(move |conn| -> NoteError<()> {
            use notes_lib::auth::AuthDelete;
            let data = data.into_inner();
            let auth = data.auth;
            let authuser = AuthUser::try_from((auth, &*conn))?;

            History::from_id(conn, id)?.delete(conn, &authuser)?;
            Ok(())
        })
        .await,
    )
}
