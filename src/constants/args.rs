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