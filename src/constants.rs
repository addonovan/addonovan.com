
//pub const CONTENT_DIR: &'static str     = "content";
pub const TEMPLATE_DIR: &'static str    = "template";

pub const MAIN_DIR: &'static str        = "content/main";
pub const RAW_DIR: &'static str         = "content/raw";
pub const BLOG_DIR: &'static str        = "content/blog";
// pub const STYLE_DIR: &'static str       = "content/raw/style";

pub const STYLE_LINK: &'static str      = "/raw/style";

pub struct Configuration {
    pub bind_address: &'static str,
}

#[cfg(debug_assertions)]
pub const CONFIG: Configuration = Configuration {
    bind_address: "127.0.0.1:8080",
};

#[cfg(not(debug_assertions))]
pub const CONFIG: Configuration = Configuration {
    bind_address: "0.0.0.0:8080",
};
