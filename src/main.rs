use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::io::stdout().write_all(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/charm_template.zip"
    )))?;

    Ok(())
}
