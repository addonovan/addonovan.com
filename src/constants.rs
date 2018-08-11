pub struct Configuration {
    pub bind_address: &'static str,
}

#[cfg(debug_assertions)]
pub const CONFIG: Configuration = Configuration {
    bind_address: "127.0.0.1:8080",
};

#[cfg(not(debug_assertions))]
pub const CONFIG: Configuration = Configuration {
    bind_address: "0.0.0.0:80",
};
