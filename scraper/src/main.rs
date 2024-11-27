use clap::Parser;
use parser::parser::update_database;
use scraper::scraper::CatalogScraper;
use std::time::Instant;

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

    #[arg(short, long, default_value_t = ("2251,2252,2254".to_string()))]
    term: String,

    #[arg(short, long, default_value_t = (".".to_string()))]
    dir: String,
}

#[tokio::main]
pub async fn main() {
    let start_time = Instant::now();
    let args = Args::parse();
    let terms: Vec<String> = args.term.split(",").map(|s| s.to_string()).collect();
    /*let terms: Vec<String> =
    Vec::from(["2251".to_string(), "2252".to_string(), "2254".to_string()]);*/
    let cache_path = &format!("{}/{}", args.dir, "cached_catalog/");
    let db_path = &format!("{}/catalog.db", args.dir);
    if (args.scrape && args.parse) || (!args.scrape && !args.parse) {
        println!(
            "Starting catalog scraping/parsing with letters: {}, and terms: {}",
            args.letters, args.term
        );
        let scraper = CatalogScraper::new(/*"catalog.db",*/ cache_path);
        scraper
            .rip_html(&args.letters, Some(args.jump.to_string()), 22, terms)
            .await
            .unwrap();
        update_database(&args.letters, cache_path, db_path, 22);
    } else {
        if args.scrape {
            println!("Starting catalog scraping with letters: {}", args.letters);
            let scraper = CatalogScraper::new(/*"catalog.db",*/ cache_path);
            scraper
                .rip_html(&args.letters, Some(args.jump.to_string()), 22, terms)
                .await
                .unwrap();
        }
        if args.parse {
            println!("Starting catalog parsing with letters: {}", args.letters);
            update_database(&args.letters, cache_path, db_path, 22);
        }
    }

    println!("\nProcessing complete! Took: {:?}", start_time.elapsed());
}
