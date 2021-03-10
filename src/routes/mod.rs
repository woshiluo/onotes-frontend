pub mod catch;
pub mod index;
pub mod post;

use notes::post::Post;

#[derive(serde::Serialize)]
pub struct TagPost {
    pub id: u32,
    pub title: String,
}

#[derive(serde::Serialize)]
pub struct PublicPost {
    pub id: u32,
    pub title: String,
    pub markdown: String,
    pub html: String,
    pub last_update: String,
    pub from_list: Vec<TagPost>,
    pub to_list: Vec<TagPost>,
}

impl TagPost {
    pub fn from_id(conn: &diesel::MysqlConnection, id: u32) -> TagPost {
        let post = Post::from_id(conn, id).unwrap();
        TagPost {
            id: post.get_id(),
            title: post.get_title().to_string(),
        }
    }
}

impl From<&Post> for TagPost {
    fn from(post: &Post) -> TagPost {
        TagPost {
            id: post.get_id(),
            title: post.get_title().to_string(),
        }
    }
}

impl PublicPost {
    pub fn from_id(conn: &diesel::MysqlConnection, id: u32) -> PublicPost {
        let post = Post::from_id(conn, id).unwrap();

        let from_list = notes::edge::Edge::get_from_list(conn, id)
            .unwrap()
            .iter()
            .map(|x| TagPost::from(&Post::from_id(conn, x.get_from()).unwrap()))
            .collect();
        let to_list = notes::edge::Edge::get_to_list(conn, id)
            .unwrap()
            .iter()
            .map(|x| TagPost::from(&Post::from_id(conn, x.get_to()).unwrap()))
            .collect();

        let mut history_list = notes::history::History::get_history(id, conn).unwrap();
        history_list.sort_by_key(|x| x.get_time());
        let history = history_list.last();

        PublicPost {
            id: post.get_id(),
            title: post.get_title().to_string(),
            markdown: post.get_markdown().to_string(),
            html: parse_markdown(post.get_markdown()),
            last_update: chrono::DateTime::<chrono::Utc>::from_utc(
                chrono::NaiveDateTime::from_timestamp(
                    match history {
                        Some(x) => x.get_time(),
                        _ => 0,
                    } as i64,
                    0,
                ),
                chrono::Utc,
            )
            .to_rfc3339(),
            from_list,
            to_list,
        }
    }
}

pub fn parse_markdown(raw_input: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};
    let options = Options::ENABLE_FOOTNOTES | Options::ENABLE_TASKLISTS;
    let parser = Parser::new_ext(raw_input, options);

    let mut output = String::new();
    html::push_html(&mut output, parser);

    output
}
