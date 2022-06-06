use std::io;
use rouille::{Request, Response, router};
use crate::{Level, LoggerFactory};
use crate::server::status::Status;

pub fn handler(request: &Request) -> Response {
    let logger = LoggerFactory::get_logger(module_path!().to_string());
    let log_ok = |req: &Request, resp: &Response, _elap: std::time::Duration| {
        logger.log(&format!(
            "{} - \"{} {}\" - {}us - {}",
            req.remote_addr(), req.method(),
            req.raw_url(), _elap.as_micros(),
            resp.status_code
        ), &Level::INFO)
    };
    let log_err = |req: &Request, _elap: std::time::Duration| {
        logger.log(&format!(
            "{} - \"{} {}\" - {}us - Handler Panicked !",
            req.remote_addr(), req.method(),
            req.raw_url(), _elap.as_micros(),
        ), &Level::ERROR)
    };
    rouille::log_custom(request, log_ok, log_err, || {
        let resp = rouille::match_assets(request, ".");
        if resp.is_success() {
            return resp
        }
        router!(request,
            (GET) (/{filename: String}) => {
                rouille::Response::text(format!(
                    "Filename: {}\nContent: {}",
                    filename,
                    rouille::percent_encoding::percent_decode(request.raw_query_string().as_bytes())
                        .decode_utf8_lossy()
                        .into_owned()
                ))
            },
            _ => {
                rouille::Response::from(Status::NotFound)
            }
        )
    })
}