pub mod edge;
pub mod history;
pub mod post;
pub mod token;
pub mod user;

type NoteError<E> = Result<E, notes_lib::NoteError>;
