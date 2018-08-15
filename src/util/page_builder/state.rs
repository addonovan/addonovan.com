use std::sync::Arc;

use controller::ControllerError;

pub enum PageState {
    Ok(Arc<String>),
    NotFound,
    InternalServerError(ControllerError),
}
