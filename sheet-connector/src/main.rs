use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use gcp_auth::CustomServiceAccount;
use gcp_auth::TokenProvider;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    sheet_id: String,
    #[arg(long)]
    table_name: String,
    #[arg(long)]
    append: bool,
}

async fn create_table(
    pool: &Pool<Postgres>,
    table_name: &String,
    append: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut readers = csv::Reader::from_path("spreadsheet.csv").unwrap();

    let headers = readers.headers().unwrap();

    let mut readers = csv::Reader::from_path("spreadsheet.csv").unwrap();
    let _first = readers.records().next().unwrap().unwrap();

    let mut queries = vec![];

    for i in 0..headers.len() {
        let q = format!("{} VARCHAR(255)", headers.get(i).unwrap());
        queries.push(q);
    }

    // for i in 0..headers.len() {
    //     let query = match first.get(i) {
    //         Some(a) => match a.parse::<i32>() {
    //             Ok(x) => format!("{} INTEGER", headers.get(i).unwrap()),
    //             Err(_) => match a.parse::<f32>() {
    //                 Ok(c) => format!("{} FLOAT", headers.get(i).unwrap()),
    //                 Err(_) => format!("{} VARCHAR(255)", headers.get(i).unwrap()),
    //             },
    //         },
    //         None => format!(""),
    //     };
    //
    //     queries.push(query);
    // }

    let query_builder = format!("CREATE TABLE {table_name} ({})", queries.join(","));
    // if let Ok(_) = sqlx::query(&query_builder).execute(pool).await {
    //     println!("Table has created")
    // }

    match sqlx::query(&query_builder).execute(pool).await {
        Ok(_) => {
            println!("Table has created")
        }
        Err(_) => {
            println!("Table already created");
            if !append {
                println!("Its Overwrite Mode. So do truncate the table");
                let truncate_builder = format!("TRUNCATE TABLE {table_name}");
                sqlx::query(&truncate_builder).execute(pool).await.unwrap();
            }
        }
    }
    Ok(())
}

async fn insert_data(
    pool: &Pool<Postgres>,
    table_name: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    // let name_table = "test";
    let mut readers = csv::Reader::from_path("spreadsheet.csv").unwrap();

    let headers = readers.headers().unwrap().clone();
    let column_names = headers.iter().collect::<Vec<_>>().join(", ");
    let placeholders: Vec<String> = (1..=headers.len())
        .map(|i| format!("${}", i)) // Use $1, $2, etc., for PostgreSQL
        .collect();
    let placeholders_str = placeholders.join(", ");

    let query_str =
        format!("insert into {table_name} ({column_names}) values ({placeholders_str})");

    for rec in readers.records() {
        let data = rec.unwrap();
        let mut query = sqlx::query(&query_str);
        for value in data.iter() {
            query = query.bind(value);
        }
        query.execute(pool).await.unwrap();
    }
    println!("Insert data has successfully");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(_) = std::fs::File::open(".env") {
        dotenv::dotenv().ok();
    }
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

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Failed connect Postgres database server");
    let table_name = cli.table_name;
    let _ = create_table(&pool, &table_name, cli.append)
        .await
        .expect("Table already created");
    let _ = insert_data(&pool, &table_name).await.unwrap();

    std::fs::remove_file("spreadsheet.csv").unwrap();
    Ok(())
}
