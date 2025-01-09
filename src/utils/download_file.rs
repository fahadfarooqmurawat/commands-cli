use reqwest::Url;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn download_file(url: &str, output_path: &str) -> Result<(), String> {
    let url = match Url::parse(url) {
        Err(_) => return Err("Failed to parse URL".into()),
        Ok(data) => data,
    };

    match url.scheme() {
        "https" | "http" => {
            // Handle HTTP/HTTPS download
            let response = reqwest::get(url.as_str())
                .await
                .map_err(|_| "Failed to download the file".to_string())?;
            let content = response
                .bytes()
                .await
                .map_err(|_| "Failed to read response body".to_string())?;

            fs::write(output_path, &content)
                .await
                .map_err(|_| "Failed to save the downloaded file".to_string())?;
        }
        "file" => {
            // Handle file protocol
            let file_path = url
                .to_file_path()
                .map_err(|_| "Invalid file URL".to_string())?;
            let mut input_file = fs::File::open(&file_path)
                .await
                .map_err(|_| "Failed to open the input file".to_string())?;
            let mut output_file = fs::File::create(output_path)
                .await
                .map_err(|_| "Failed to create the output file".to_string())?;

            let mut buffer = Vec::new();
            input_file
                .read_to_end(&mut buffer)
                .await
                .map_err(|_| "Failed to read from the input file".to_string())?;
            output_file
                .write_all(&buffer)
                .await
                .map_err(|_| "Failed to write to the output file".to_string())?;
        }
        _ => return Err("Unsupported URL scheme".into()),
    }

    Ok(())
}
