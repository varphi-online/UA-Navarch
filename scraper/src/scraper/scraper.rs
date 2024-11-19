use super::client::ScraperClient;
use futures::future;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use regex::*;
use sqlite::*;
use std::fs::{create_dir, create_dir_all};
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct CatalogScraper {
    base_url: String,
    cache_path: String,
}

impl CatalogScraper {
    pub fn new(cache_path: &str) -> Self {
        Self {
            base_url: "https://catsched.cv.studentcenter.arizona.edu/psc/pubsaprd/UA_CATALOG/HRMS/c/ESTABLISH_COURSES.SSS_BROWSE_CATLG.GBL".to_string(),
            cache_path: cache_path.to_string(),
        }
    }

    pub async fn rip_html(
        &self,
        letters: &str,
        start_idx: Option<String>,
        thread_count: usize,
        terms: Vec<String>,
    ) -> Result<()> {
        // Create cache directory
        if let Err(e) = create_dir(&self.cache_path) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                println!("Failed to create cache directory: {}", e);
            }
        }

        let letters: Vec<char> = letters.chars().collect();
        let first_iter = Arc::new(AtomicBool::new(true));
        let multi_progress = Arc::new(MultiProgress::new());
        let unique = Arc::new(AtomicU64::new(0));
        let terms = Arc::new(terms);

        // Process letters in chunks
        for chunk in letters.chunks(thread_count) {
            let mut handles = Vec::new();

            for &letter in chunk {
                let url = self.base_url.clone();
                let root = self.cache_path.clone();
                let start_idx = start_idx.clone();
                let first_iter = Arc::clone(&first_iter);
                let multi_progress = Arc::clone(&multi_progress);
                let unique = Arc::clone(&unique);
                let terms = Arc::clone(&terms);

                let handle = tokio::spawn(async move {
                    let start: usize = if first_iter
                        .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
                        .is_ok()
                    {
                        start_idx.unwrap_or("0".to_string()).parse().unwrap_or(0)
                    } else {
                        0
                    };

                    if start != 0 {
                        println!("Jumping to {start}");
                    }

                    //println!("Starting processing for letter: {}", letter);
                    let letter_path = format!("{}{}_deps", root, letter);

                    // Create letter directory
                    if let Err(e) = create_dir(&letter_path) {
                        if e.kind() != std::io::ErrorKind::AlreadyExists {
                            println!("Failed to create letter directory: {}", e);
                            return;
                        }
                    }

                    let mut client = ScraperClient::new(&url, Arc::clone(&unique));
                    client.initialize_session().await;
                    let spinner = multi_progress.add(ProgressBar::new_spinner());
                    spinner.set_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.green} {msg}")
                            .unwrap_or(ProgressStyle::default_spinner()),
                    );

                    spinner.set_message(format!("Fetching courses under \"{}\"", letter));
                    spinner.enable_steady_tick(Duration::from_millis(50));

                    let expand_text = client.expand_departments(letter).await;
                    spinner.finish_and_clear();

                    let course_ids =
                        get_highest_course_number(expand_text.as_str()).unwrap_or(0) as usize;

                    let progress_bar = multi_progress.add(ProgressBar::new(std::cmp::min(
                        (course_ids - start) as u64,
                        course_ids as u64,
                    )));
                    progress_bar.set_style(
                    ProgressStyle::with_template(
                        "Elapsed: [{elapsed_precise}] | ETA: [{eta_precise}] | {msg}\n{wide_bar} [{pos}/{len}]",
                    )
                    .unwrap_or(ProgressStyle::default_bar())
                );
                    let mut current_id: usize = start;

                    while current_id < course_ids {
                        progress_bar.set_message(format!("Processing letter: {}", letter));
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        create_dir_all(format!("{}/course_{current_id}/", &letter_path))
                            .unwrap_or(());

                        let mut retry_count = 0;
                        const MAX_RETRIES: u32 = 3;

                        loop {
                            let data_integrity_failure = client
                                .extract_class_details(
                                    current_id.to_string().as_str(),
                                    format!("{}/course_{current_id}/", &letter_path),
                                    &letter,
                                    &terms,
                                )
                                .await;

                            if !data_integrity_failure {
                                // Success - move to next course
                                break;
                            }
                            progress_bar.set_message(format!("Processing letter: {} | Data integrity failure for course {}. Attempt {}/{}. Recreating client and retrying...", letter,current_id, retry_count, MAX_RETRIES));

                            // Recreate client and reinitialize session
                            client = ScraperClient::new(&url, Arc::clone(&unique));
                            client.initialize_session().await;
                            client.expand_departments(letter).await;
                            retry_count += 1;
                            if retry_count > MAX_RETRIES {
                                /*
                                println!(
                                    "Failed to process course {} after {} retries.",
                                    current_id, MAX_RETRIES
                                );
                                */
                                progress_bar.set_message("skipping");
                                break;
                            }

                            // Add a small delay before retry
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }

                        progress_bar.inc(1);
                        current_id += 1;
                    }
                    /*
                                        for id in start..course_ids {
                                            tokio::time::sleep(Duration::from_millis(500)).await;
                                            create_dir_all(format!("{}/course_{id}/", &letter_path)).unwrap();

                                            let data_integrity_failure = client
                                                .extract_class_details(
                                                    id.to_string().as_str(),
                                                    format!("{}/course_{id}/", &letter_path),
                                                )
                                                .await;

                                            progress_bar.inc(1);
                                        }
                    */
                    progress_bar.finish_and_clear();
                });

                handles.push(handle);
            }

            // Wait for all handles in this chunk to complete
            let _ = future::try_join_all(handles.into_iter()).await;
        }

        Ok(())
    }
}

fn get_highest_course_number(text: &str) -> Option<u32> {
    if let Ok(re) = Regex::new(r"CRSE_TITLE\$\d+") {
        return re
            .find_iter(text)
            .filter_map(|m| m.as_str().split('$').last()?.parse::<u32>().ok())
            .max();
    }
    None
}
