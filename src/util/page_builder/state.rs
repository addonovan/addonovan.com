#[derive(PartialEq)]
pub enum PageState {
    Ok,
    NotFound,
    InternalServerError,
}
