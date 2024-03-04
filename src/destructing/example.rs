#[allow(dead_code)]
mod example {
    enum CustomError {
        FileReadError(std::io::Error),
        RequestError(reqwest::Error),
    }

    fn example_error() -> Result<(), CustomError> {
        // let response = reqwest::blocking::get("https://google.com")?;

        Ok(())
    }
}
