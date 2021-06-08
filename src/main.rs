use std::env;

use futures::StreamExt;
use regex::Regex;
use reqwest;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                let res = match get_resp(data.to_string()) {
                    Ok(res) => res,
                    Err(_err) => String::from("ERROR"),
                };

                // Answer message with "Hi".
                api.send(message.text_reply(format!(
                    "Hi, {}! Here's your link: {}",
                    &message.from.first_name, res
                )))
                .await?;
            }
        }
    }
    Ok(())
}

//TODO: error handling
fn get_resp(url: String) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: check data before sending request
    if !url.contains("https") {
        Ok(String::from("N/A"))
    } else {
        // cba adding reddit response struct
        let url_pat = r"(.+)(/.+)$";
        let url_res = Regex::new(&url_pat).unwrap().captures(&url).unwrap();
        let mut real_url = String::from(url_res.get(1).unwrap().as_str());
        real_url.push_str(".json");
        println!("{:?}", real_url);

        let resp = reqwest::blocking::get(real_url)?.text().unwrap();
        let pat = "fallback_url\": \"(.*?)\"";
        let pat2 = "\"(.*?)\"";

        let cap = Regex::new(&pat).unwrap().captures(&resp).unwrap();
        let res = cap.get(0).unwrap().as_str();
        println!("{:?}", res);

        let cap2 = Regex::new(&pat2).unwrap().captures(&res).unwrap();
        println!("{:?}", cap2.get(1).unwrap().as_str());
        Ok(res.to_string())
    }
}
