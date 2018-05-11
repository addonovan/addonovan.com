
pub struct Configuration
{
    pub address: &'static str,
    pub debug: bool,
}

#[cfg(debug_assertions)]
pub const CONFIG: Configuration = Configuration {
    address: "127.0.0.1:8080",
    debug: true,
};

#[cfg(not(debug_assertions))]
pub const CONFIG: Configuration = Configuration {
    address: "0.0.0.0:80",
    debug: false,
};
