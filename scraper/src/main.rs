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
    /// Scrape catalog without parsing data
    #[arg(short, long, default_value_t = false)]
    scrape: bool,

    /// Parse pre-scraped data only
    #[arg(short, long, default_value_t = false)]
    parse: bool,

    /// Capital letter(s) concatenated with no spacing
    #[arg(short, long, default_value_t = ("ABCDEFGHIJKLMNOPRSTUVW".to_string()))]
    letters: String,

    /// Debug flag, meant to skip a number of classes in the event of a failed scrape
    #[arg(short, long, default_value_t = 0)]
    jump: u32,

    /// Integer(s) concatenated with commas that correspond to terms within UA's website
    #[arg(short, long, default_value_t = ("2251,2252,2254".to_string()))]
    term: String,

    /// Root directory to store cached data and the output database file
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
