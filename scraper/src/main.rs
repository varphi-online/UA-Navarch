use parser::parser::update_database;
use scraper::scraper::CatalogScraper;
use std::env;
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

#[tokio::main]
pub async fn main() {
    let cache_path = "./cached_catalog/";
    let letters: String = env::args()
        .nth(1)
        .unwrap_or("ABCDEFGHIJKLMNOPRSTUVW".to_string());
    let start_idx: Option<String> = env::args().nth(2);

    println!("Starting catalog scraping with letters: {}", letters);

    let scraper = CatalogScraper::new(/*"catalog.db",*/ cache_path);
    scraper.rip_html(&letters, start_idx, 22).await.unwrap();
    update_database(&letters, "catalog.db", 22);
    println!("\nProcessing complete!");
}
