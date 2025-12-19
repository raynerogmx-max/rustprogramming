use serde::Deserialize;
use std::error::Error;
use std::io::Read; // required for read_to_end()

#[derive(Debug)]
enum DogError {
    Network(String),
    Json(String),
    ImageBytes(String),
}

impl std::fmt::Display for DogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DogError {}

#[derive(Deserialize)]
struct DogApiResponse {
    message: String,
    status: String,
}

fn get_random_dog_url() -> Result<String, DogError> {
    let url = "https://dog.ceo/api/breeds/image/random";

    let resp = ureq::get(url).call()
        .map_err(|e| DogError::Network(e.to_string()))?;

    // Use a reader
    let reader = resp.into_reader();
    let json: DogApiResponse = serde_json::from_reader(reader)
        .map_err(|e| DogError::Json(e.to_string()))?;

    Ok(json.message)
}

fn download_image(url: &str) -> Result<Vec<u8>, DogError> {
    let resp = ureq::get(url).call()
        .map_err(|e| DogError::Network(e.to_string()))?;

    let mut reader = resp.into_reader();
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)
        .map_err(|e| DogError::ImageBytes(e.to_string()))?;

    Ok(buf)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Fetching random dog image URL...");

    let dog_url = get_random_dog_url()?;
    println!("Dog image URL: {}", dog_url);

    println!("Downloading image bytes...");
    let bytes = download_image(&dog_url)?;
    println!("Downloaded {} bytes", bytes.len());

    Ok(())
}
