use reqwest::Client;
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use tokio::time::{sleep, Duration};
use walkdir::WalkDir;

// This function sends a HTTP request to upload the DICOM file to the server
async fn upload_dicom_file(client: &Client, server_address: &str, path: &Path) {
    let data = fs::read(path).unwrap();
    client.post(server_address).body(data).send().await.unwrap();
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Read the directory path and Orthanc server address from environment variables
    let directory_path = env::var("DIRECTORY_PATH").expect("DIRECTORY_PATH must be set");
    let server_address = env::var("ORTHANC_ADDRESS").expect("ORTHANC_ADDRESS must be set");

    // Read the sleep duration from an environment variable, or default to 5 seconds
    let sleep_duration_secs = env::var("SLEEP_DURATION")
        .map(|v| u64::from_str(&v).expect("SLEEP_DURATION must be a valid integer"))
        .unwrap_or(5);

    loop {
        // Scan directory for .dcm files
        for entry in WalkDir::new(&directory_path) {
            let entry = entry.unwrap();
            if entry.file_name().to_string_lossy().ends_with(".dcm") {
                // If it's a .dcm file, upload it to the Orthanc DICOM server
                let path = entry.path();
                upload_dicom_file(&client, &server_address, &path).await;

                // Delete the file regardless of whether the upload was successful
                fs::remove_file(path).unwrap();
            }
        }

        // Sleep for the specified duration before next scan
        sleep(Duration::from_secs(sleep_duration_secs)).await;
    }
}
