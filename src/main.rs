use chrono::prelude::*;
use chrono_tz::Europe::Prague;
use log::info;
use prost::Message;
//use std::env;
use serde_json;
use std::error::Error;

pub mod transit_realtime {
    include!(concat!(env!("OUT_DIR"), "/transit_realtime.rs"));
}

async fn fetch_feed_msg(
    url: &str,
    params: &[(&str, &str)],
) -> Result<transit_realtime::FeedMessage, Box<dyn Error>> {
    let res = reqwest::get(reqwest::Url::parse_with_params(url, params).unwrap()).await?;

    info!(
        "Fetched: {}, Response: {:?} {}",
        url,
        res.version(),
        res.status()
    );

    let body = res.bytes().await?;
    let data = transit_realtime::FeedMessage::decode(body)?;

    let naive =
        NaiveDateTime::from_timestamp_opt(data.header.timestamp.unwrap() as i64, 0).unwrap();
    let timestamp = Prague.from_utc_datetime(&naive);
    info!("Data from: {}", timestamp);
    Ok(data)
}

async fn fetch_json(url: &str, params: &[(&str, &str)]) -> Result<serde_json::Value, Box<dyn Error>> {
    let res = reqwest::get(reqwest::Url::parse_with_params(url, params).unwrap()).await?;

    info!(
        "Fetched: {}, Response: {:?} {}",
        url,
        res.version(),
        res.status()
    );

    let body = res.text().await?;
    let data = serde_json::from_str(&body).unwrap();
    Ok(data)
}

async fn get_pid_feed() -> Result<(), Box<dyn Error>> {
    let pid_feed_url = "https://api.golemio.cz/v2/vehiclepositions/gtfsrt/pid_feed.pb";
    let _msg = fetch_feed_msg(pid_feed_url, &[]);
    Ok(())
}

async fn get_departure_board() -> Result<serde_json::Value, Box<dyn Error>> {
    let dep_board_url = "https://api.golemio.cz/v2/pid/departureboards";
    let params = [("names", "Albertov")];
    let msg = fetch_json(dep_board_url, &params).await?;
    Ok(msg)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger level
    env_logger::builder()
        .filter(None, log::LevelFilter::Debug)
        .init();
    //let _token = env::var("GOLEMIO_API_KEY_TOKEN");

    let dep_board = get_departure_board().await?;
    println!("{}", dep_board["stops"]["stop_name"]);

    Ok(())
}
