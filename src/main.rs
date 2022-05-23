use argh::FromArgs;
use std::collections::HashMap;
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply, reject, reply, http::StatusCode};

fn default_port() -> u16 {
    3030
}

#[derive(FromArgs)]
/// approxy - Apollo proxy for NGtrend
struct Args {
    /// listening port (default: 3030)
    #[argh(option, default="default_port()", short='p')]
    port: u16,

    /// base url for the Apollo service
    #[argh(option, short='u')]
    url: String,

    /// messaging secret
    #[argh(option, short='s')]
    secret: String,
}

#[derive(Debug)]
struct BadGateway;
impl reject::Reject for BadGateway {}

#[derive(Debug)]
struct BadRequest;
impl reject::Reject for BadRequest {}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.is_not_found() {
        Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else if let Some(_) = err.find::<BadGateway>() {
        Ok(reply::with_status("BAD_GATEWAY", StatusCode::BAD_GATEWAY))
    } else if let Some(_) = err.find::<BadRequest>() {
        Ok(reply::with_status("BAD_REQUEST", StatusCode::BAD_REQUEST))
    } else {
        Ok(reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::INTERNAL_SERVER_ERROR))
    }
}

async fn process_request(form: HashMap<String, String>, url: String, secret: String) -> Result<impl Reply, Rejection> {
    if form.get("msg").is_none() || form.get("tel").is_none() {
        return Err(reject::custom(BadRequest));
    }

    let url = reqwest::Url::parse_with_params(url.as_str(), &[
        ("sender", form.get("tel").unwrap()),
        ("text", form.get("msg").unwrap()),
        ("secret", &secret),
        ]);

    if url.is_err() {
        return Err(reject::custom(BadRequest));
    }

    let resp = reqwest::get(url.unwrap()).await;
    if resp.is_ok() {
        let body = resp.unwrap().text().await;
        if body.is_ok() {
            return Ok(format!("<root><status>0</status><message>{}</message></root>", body.unwrap().trim()));
        }
    }

    Err(reject::custom(BadGateway))
}

#[tokio::main]
async fn main() {
    let args: Args = argh::from_env();

    let messages = warp::any()
        .and(warp::post())
        .and(warp::body::form())
        .and_then(move |form| {
            let url = args.url.clone();
            let secret = args.secret.clone();
            async move {
                process_request(form, url, secret).await
            }
        })
        .with(warp::reply::with::header("content-type", "text/xml; charset=utf-8"))
        .recover(handle_rejection);

    warp::serve(messages)
        .run(([127, 0, 0, 1], args.port))
        .await;
}
