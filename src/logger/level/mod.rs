#[derive(strum_macros::Display)]
pub enum Level {
    #[strum(serialize = "ERROR")]
    ERROR,
    #[strum(serialize = "!")]
    WARNING,
    #[strum(serialize = "*")]
    INFO,
    #[strum(serialize = "CRITICAL")]
    CRITICAL,
    #[strum(serialize = "ERROR")]
    EXCEPTION,
    #[strum(serialize = "-")]
    FAILURE,
    #[strum(serialize = "*")]
    HEXDUMP,
    #[strum(serialize = "")]
    INDENTED,
    #[strum(serialize = "x")]
    PROGRESS,
    #[strum(serialize = "+")]
    SUCCESS,
    #[strum(serialize = "x")]
    WAITFOR,
}