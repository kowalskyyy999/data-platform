use std::io::Write;
use std::path::PathBuf;

use gcp_auth::CustomServiceAccount;
use gcp_auth::TokenProvider;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    sheet_id: String,
}

#[tokio::main]
async fn main() {
    std::fs::File::open("credentials.json").expect("credentials.json not found");

    let cli = Cli::parse();

    let creds = PathBuf::from("credentials.json");
    let service_account = CustomServiceAccount::from_file(creds).unwrap();
    let scopes = &["https://www.googleapis.com/auth/spreadsheets"];
    let token = service_account.token(scopes).await.unwrap();

    let url = format!(
        "https://docs.google.com/spreadsheets/d/{}/export?format=csv",
        cli.sheet_id
    );

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .bearer_auth(token.as_str())
        .send()
        .await
        .unwrap();

    let mut file = std::fs::File::create("spreadsheet.csv").unwrap();
    let _ = file.write_all(&response.bytes().await.unwrap());
}
