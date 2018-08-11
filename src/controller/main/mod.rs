pub const CONTENT_DIR: &'static str = "res/content";
pub const TEMPLATE_DIR: &'static str = "res/template";

mod builder;

mod controller;
pub use self::controller::MainController;
