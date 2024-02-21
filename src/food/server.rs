use reqwest::Url;
use serde::Deserialize;
use std::env;
use std::sync::mpsc::Sender;
use tide::Request;

fn get_tracking_url() -> String {
    env::var("WOLT_TRACKING_URL").unwrap_or_else(|_| String::from("http://localhost:9000/"))
}

pub async fn food_endpoint_server(tx: Sender<Url>) -> tide::Result<()> {
    let mut app = tide::new();
    let food_html = include_str!("index.html");

    app.at("/food").get(move |_| async move {
        let response = tide::Response::builder(200)
            .content_type(tide::http::mime::HTML)
            .body(food_html)
            .build();
        Ok(response)
    });

    app.at("/tracking")
        .post(move |req| start_tracking(tx.clone(), req));

    app.listen("0.0.0.0:1337").await?;
    Ok(())
}

async fn start_tracking(tx: Sender<Url>, mut req: Request<()>) -> tide::Result {
    let tracking: Tracking = req.body_form().await?;
    let wolt_tracking_url = get_tracking_url();
    //TOOD: Avoid unwrap
    let tracking_id = tracking.url.path_segments().unwrap().last().unwrap();
    let api_url = Url::parse(&format!("{}{}", wolt_tracking_url, tracking_id)).unwrap();
    // Pass tracking url to the worker thread
    tx.send(api_url).unwrap();
    Ok(format!("Got it! Tracking food delivery").into())
}

#[derive(Debug, Deserialize)]
struct Tracking {
    url: Url,
}