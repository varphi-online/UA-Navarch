use clap::Parser;
use parser::parser::update_database;
use scraper::scraper::CatalogScraper;
/*
#[derive(Debug, Clone)]
struct CourseDetails {
    course_number: Option<String>,
    title: Option<String>,
    description: Option<String>,
    units: Option<String>,
    prerequisites: Option<String>,
    department: Option<String>,
}
*/

mod parser;
mod scraper;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    scrape: bool,

    #[arg(short, long, default_value_t = false)]
    parse: bool,

    /// Name of the person to greet
    #[arg(short, long, default_value_t = ("ABCDEFGHIJKLMNOPRSTUVW".to_string()))]
    letters: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 0)]
    jump: u32,
}

#[tokio::main]
pub async fn main() {
    let args = Args::parse();
    let cache_path = "./cached_catalog/";
    if (args.scrape && args.parse) || (!args.scrape && !args.parse) {
        println!(
            "Starting catalog scraping/parsing with letters: {}",
            args.letters
        );
        let scraper = CatalogScraper::new(/*"catalog.db",*/ cache_path);
        scraper
            .rip_html(&args.letters, Some(args.jump.to_string()), 22)
            .await
            .unwrap();
        update_database(&args.letters, cache_path, "catalog.db", 22);
    } else {
        if args.scrape {
            println!("Starting catalog scraping with letters: {}", args.letters);
            let scraper = CatalogScraper::new(/*"catalog.db",*/ cache_path);
            scraper
                .rip_html(&args.letters, Some(args.jump.to_string()), 22)
                .await
                .unwrap();
        }
        if args.parse {
            println!("Starting catalog parsing with letters: {}", args.letters);
            update_database(&args.letters, cache_path, "catalog.db", 22);
        }
    }

    println!("\nProcessing complete!");
}
