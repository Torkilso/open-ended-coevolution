use std::fs;
use std::io::Write;

#[allow(dead_code)]
pub fn write_text_to_file(path: String, text: &[u8]) -> std::io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(text)?;
    Ok(())
}
