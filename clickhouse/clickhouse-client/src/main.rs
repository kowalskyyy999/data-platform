use clap::Parser;
use clickhouse;
use dotenv::dotenv;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    query: String,
    #[arg(long, default_value_t = 10)]
    max_retry: i32,
    #[arg(long, default_value_t = 30)]
    query_timeout: u64,
}

#[tokio::main]
async fn main() {
    if let Ok(_) = std::fs::File::open(".env") {
        dotenv().ok();
    }
    let cli = Cli::parse();

    let mut retry_count = 0;

    let client = clickhouse::Client::default()
        .with_url(
            std::env::var("CLICKHOUSE_HOST").unwrap_or("http://clickhouse-server:8123".to_string()),
        )
        .with_user(std::env::var("CLICKHOUSE_USER").unwrap_or("admin".to_string()))
        .with_password(std::env::var("CLICKHOUSE_PASSWORD").unwrap_or("password".to_string()));

    let query = cli.query;

    while let Err(e) = client.query(&query).execute().await {
        retry_count += 1;
        println!("Error: {e}");
        if retry_count > cli.max_retry {
            eprintln!("Failed execute query !!!");
            break;
        }
        let _ = tokio::time::sleep(tokio::time::Duration::from_secs(cli.query_timeout)).await;
    }
    println!("Success execute query !!!")
}
