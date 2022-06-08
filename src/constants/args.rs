pub mod encode {
    pub const NAME: &str = "Encode";
    pub const LONG: &str = "encode";
    pub const HELP: &str = "Order of encoding applied to the content";
    pub const SHORT: char = 'e';
    pub const VALUE_NAME: &str = "encoding";
    pub const POSSIBLE_VALUES: &[&str] = &[
        "b64", "zlib",
        "bzip2", "rot13"
    ];
}

pub mod host {
    pub const NAME: &str = "Host";
    pub const HELP: &str = "Host Listener";
    pub const VALUE_NAME: &str = "ip|hostname";
}

pub mod port {
    pub const NAME: &str = "Port";
    pub const HELP: &str = "Port Listener";
    pub const LONG: &str = "port";
    pub const SHORT: char = 'p';
    pub const VALUE_NAME: &str = "port";
}