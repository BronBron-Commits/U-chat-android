use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

pub async fn login_request(base: &str, username: String, password: String)
    -> Result<String, String>
{
    let client = reqwest::Client::new();
    let url = format!("{}/login", base);

    let res = client.post(&url)
        .json(&LoginPayload { username, password })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status() != 200 {
        return Err("Invalid username or password".into());
    }

    let json: serde_json::Value = res.json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(json["token"].as_str().unwrap_or("").to_string())
}
