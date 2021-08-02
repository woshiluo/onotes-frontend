pub mod api;
pub mod routes;

#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;
use rocket_sync_db_pools::{database, diesel};

use crate::api::edge::{get_from_list, get_to_list, update_from_list, update_to_list};
use crate::api::history::{delete_history, get_history};
use crate::api::post::{delete_post, new_post, return_post, update_post};
use crate::api::token::get_token;
use crate::api::user::{register_user, return_user, update_user};
use crate::routes::catch::not_found;
use crate::routes::history::{list_history, show_history};
use crate::routes::index::{get_sw, index, login, static_files};
use crate::routes::post::{edit_post, view_post};

#[database("db")]
pub struct DbConn(diesel::MysqlConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                static_files,
                get_token,
                register_user,
                update_user,
                return_user,
                new_post,
                update_post,
                return_post,
                delete_post,
                get_history,
                delete_history,
                get_from_list,
                get_to_list,
                update_from_list,
                update_to_list,
                index,
                login,
                get_sw,
                view_post,
                edit_post,
                list_history,
                show_history
            ],
        )
        .attach(Template::fairing())
        .attach(DbConn::fairing())
        .register("/", catchers![not_found])
}
