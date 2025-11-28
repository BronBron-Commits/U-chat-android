use std::fs;

pub fn save_token(token: &str) -> Result<(), String> {
    fs::write("token.txt", token).map_err(|e| e.to_string())
}

pub fn load_token() -> Option<String> {
    fs::read_to_string("token.txt").ok()
}
