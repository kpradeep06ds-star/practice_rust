fn main() -> Result<(), std::io::Error> {
    let _f = std::fs::File::open("missing.txt")?;
    Ok(())
}