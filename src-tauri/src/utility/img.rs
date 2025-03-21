use std::fs;

pub fn read_image_file(path: String) -> Result<String, String> {
    let bytes = fs::read(&path).map_err(|e| e.to_string())?;
    let base64 = base64::encode(&bytes);
    Ok(format!("data:image/png;base64,{}", base64))
}
