#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod storage;

#[tauri::command]
async fn login_request(username: String, password: String) -> Result<String, String> {
    let token = auth::login("http://10.1.10.83:9100", username, password).await?;
    storage::save_token(&token)?;
    Ok(token)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            login_request
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri");
}
