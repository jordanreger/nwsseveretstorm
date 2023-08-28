pub async fn latest_warning() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let res = client
        .get("https://api.weather.gov/alerts/active?status=actual&message_type=alert&event=Severe%20Thunderstorm%20Warning&limit=1")
        .header("User-Agent", "nwsseveretstorm.bsky.social")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let features = &res["features"];

    let headline = match features.as_array() {
        Some(x) if x.is_empty() => "No current severe thunderstorm warnings".to_string(),
        _ => features[0]["properties"]["headline"]
            .to_string()
            .replace("\"", ""),
    };

    let mut warning = headline.clone();

    if !headline.eq(&String::from("No current severe thunderstorm warnings")) {
        let area_desc = &features[0]["properties"]["areaDesc"]
            .to_string()
            .replace("\"", "");
        let station =
            &features[0]["properties"]["parameters"]["AWIPSidentifier"][0].to_string()[4..7];
        warning = format!(
            "{station} - {area_desc}

{headline}"
        );
    }

    Ok(warning)
}
