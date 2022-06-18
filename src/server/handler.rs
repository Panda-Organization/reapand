use std::fs::{create_dir_all, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use bzip2::read::BzDecoder;
use flate2::read::ZlibDecoder;
use itertools::Itertools;
use rouille::{Request, Response, router};
use crate::{APP_MATCHES, constants, generate_app, Level, LoggerFactory};
use crate::server::status::Status;

pub fn handler(request: &Request) -> Response {
    let encodings = APP_MATCHES.values_of(constants::args::encode::NAME)
                                                 .unwrap()
                                                 .map(String::from).collect::<Vec<String>>();

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
                let exfiltrated_encoded = rouille::percent_encoding::percent_decode(request.raw_query_string().as_bytes())
                                    .decode_utf8_lossy()
                                    .into_owned();
                if exfiltrated_encoded.is_empty() {
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

                let new_path = format!(
                    "{}/{}",
                    APP_MATCHES.value_of(constants::args::directory::NAME)
                               .unwrap()
                               .trim_end_matches("/"),
                    &new_file
                );

                let path = Path::new(&new_path);
                let prefix = path.parent().unwrap();
                create_dir_all(prefix).unwrap();

                let content = encodings.iter()
                    .map(|s| s.clone().into_bytes())
                    .collect::<Vec<Vec<u8>>>()
                    .iter()
                    .fold(exfiltrated_encoded.clone().into_bytes(), |acc, x| match String::from_utf8_lossy(&x).as_ref() {
                        "b64" => {
                            if let Ok(content) = base64::decode(acc) {
                                content
                            } else {
                                b"".to_vec()
                            }
                        }
                        "zlib" => {
                            let mut d = ZlibDecoder::new(acc.as_slice());
                            let mut v = Vec::new();
                            if let Ok(content) = d.read_to_end(&mut v) {
                                v.to_vec()
                            } else {
                                b"".to_vec()
                            }
                        }
                        "bzip2" => {
                            let mut d = BzDecoder::new(acc.as_slice());
                            let mut v = Vec::new();
                            if let Ok(content) =  d.read_to_end(&mut v) {
                                v.to_vec()
                            } else {
                                b"".to_vec()
                            }
                        }
                        _ => {
                            b"".to_vec()
                        }
                    });

                if !content.is_empty() {
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