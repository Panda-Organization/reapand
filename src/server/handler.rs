use std::io;
use itertools::Itertools;
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
                let exfiltrated = rouille::percent_encoding::percent_decode(request.raw_query_string().as_bytes())
                                    .decode_utf8_lossy()
                                    .into_owned();
                let newFile = filename.trim_matches(&['_', '.', ' ', '/', '\\'] as &[_])
                                      .replace(&['\\', '/'][..], "")
                                      .chars()
                                      .dedup_by(|x, y| {
                                            x == &'_' && y == &'_' ||
                                            x == &'.' && y == &'.'
                                      })
                                      .collect::<Vec<char>>()
                                      .into_iter()
                                      .collect::<String>()
                                      .split("_")
                                      .collect::<Vec<&str>>()
                                      .join("/");
                let exfiltrated = base64::decode(exfiltrated);
                if let Ok(content) = exfiltrated {
                  return rouille::Response::text(format!(
                    "Filename: {}\nContent: {}",
                    newFile,
                    String::from_utf8_lossy(&content)
                  ));
                } else {
                    rouille::Response::from(Status::NotFound)
                }
            },
            _ => {
                rouille::Response::from(Status::NotFound)
            }
        )
    })
}