use crate::services::api;
use crate::services::storage;

#[tauri::command]
pub async fn login(username: String, password: String) -> Result<String, String> {
    let token = api::login_request(
        "http://10.1.10.83:9100",
        username,
        password
    ).await?;

    storage::save_token(&token)?;
    Ok(token)
}
