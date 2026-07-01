use std::process::Command;

pub fn download(url: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("curl")
        .args(["-o", output_path, url])
        .status()?;
    Ok(())
}

pub fn extract(input_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("7z").args(["x", input_path]).status()?;
    Ok(())
}
