use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::Path;
use itertools::Itertools;
use rouille::{Request, Response, router};
use crate::{constants, generate_app, Level, LoggerFactory};
use crate::server::status::Status;

pub fn handler(request: &Request) -> Response {
    let matches = generate_app().get_matches();
    if let Some(e) = matches.values_of(constants::args::encode::NAME) {
        println!("From Handler");
        println!("{:?}", e.map(String::from).collect::<Vec<String>>());
    }

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
                if exfiltrated.is_empty() {
                    return rouille::Response::from(Status::NotFound);
                }
                let new_file = filename.trim_matches(&['_', '.', ' ', '/', '\\'] as &[_])
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

                let new_path = format!("files/{}", &new_file);
                let path = Path::new(&new_path);
                println!("{:?}", &path);
                let prefix = path.parent().unwrap();
                create_dir_all(prefix).unwrap();

                let exfiltrated = base64::decode(exfiltrated);
                if let Ok(content) = exfiltrated {
                  let mut f = OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(&path)
                    .expect(&*format!("Unable to open file [{:?}]", &path));

                  f.write_all(&content)
                    .expect(&*format!("Cannot write log message to file [{:?}]", &path));
                  return rouille::Response::text(format!(
                    "Filename: {}\nContent: {}",
                    new_file,
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