mod api;
mod pb;

use crate::pb::Paste;
// use std::panic;
use worker::*;
use crate::api::aes::aes_decrypt;

static PREFIX: &str = "##PasteMe##";

// extern crate console_error_panic_hook;

// use regex::Regex;
const HOST: &'static str = "https://bone.saltedfish.fun/_api/paste/";

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    // panic::set_hook(Box::new(console_error_panic_hook::hook));

    // let path_regex: Regex = Regex::new(r"/paste/(?P<pasteID>[^/]+)/?$").unwrap();
    let mut password = String::new();

    console_log!(
        "{} {}, located at: {:?}, within: {}",
        req.method().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
    if !matches!(req.method(), Method::Get) {
        return Response::error("Method not implemented", 405);
    }
    let query = req.url().unwrap();
    let mut query = query.query_pairs();
    if query.count() > 0 {
        while let Some(pair) = query.next() {
            if pair.0 == "p" {
                password = pair.1.to_string();
            }
        }
    }

    let mut path = req.path();
    if !path.starts_with("/paste/") {
        return Response::error("Not Found", 404);
    }
    if path.ends_with("/") {
        path.pop();
    }
    if let Some(pasteID) = req.path().split("/").last() {
        console_log!("{:?}", pasteID);
        if pasteID.is_empty() {
            return Response::error("Invalid paste ID", 400);
        } else {
            let mut headers = Headers::new();
            headers.append("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.149 Safari/537.36").expect("");
            let req = Request::new_with_init(
                format!("{}{}?json=true", HOST, pasteID).as_str(),
                RequestInit::new()
                    .with_headers(headers)
                    .with_method(Method::Get),
            )
                .unwrap();
            console_log!("REQUEST {:?}",req.url());
            let data = Fetch::Request(req).send().await;
            if data.is_err() {
                return Response::error("Fetch failed", 500);
            }
            let mut data = data.unwrap();
            // console_log!("{:?}",data);

            let data = data.text().await.unwrap();
            // console_log!("{:?}",data);
            let paste = Paste::from_base64(data).unwrap();
            console_log!("{:?}", paste);
            // console_log!("{:?}", paste.status);

            if paste.status != 200 && paste.status != 401 {
                return Response::error(format!("Fetch Error,Error {}", paste.error), paste.status as u16);
            }
            if !paste.password {
                return Response::ok(paste.content);
            } else {
                if password.is_empty() {
                    return Response::error(format!("Protected Paste"), 403);
                } else {
                    let result = aes_decrypt(paste.content.as_str(), password.as_str());
                    if result.is_err() {
                        return Response::error(format!("Decrypt Failed"), 403);
                    } else {
                        let result = result.unwrap();
                        if !result.starts_with(PREFIX) {
                            return Response::error(format!("Decrypt Failed"), 403);
                        } else {
                            let result = result.strip_prefix(PREFIX).unwrap();
                            return Response::ok(result);
                        }
                    }
                }
            }
        }
    } else {
        return Response::error("Bad Request", 400);
    }

    Response::error("Bad Request", 400)
}
