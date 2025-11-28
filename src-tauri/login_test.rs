use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct LoginResponse {
    token: Option<String>,
}

#[tokio::main]
async fn main() {
    let username = "testuser";
    let password = "testpass";

    println!("Sending login request…");

    let client = reqwest::Client::new();

    let res = client
        .post("http://10.1.10.83:9100/login")
        .json(&json!({
            "username": username,
            "password": password
        }))
        .send()
        .await;

    let res = match res {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Request failed: {}", e);
            return;
        }
    };

    println!("Status: {}", res.status());

    let body = res.text().await.unwrap_or_else(|_| "<no body>".into());
    println!("Raw response: {}", body);

    // Try parsing as JSON
    if let Ok(parsed_json) = serde_json::from_str::<serde_json::Value>(&body) {
        if let Some(token) = parsed_json.get("token").and_then(|t| t.as_str()) {
            println!("Extracted token: {}", token);
        } else {
            println!("No token field found.");
        }
    } else {
        println!("Body is not valid JSON.");
    }
}
