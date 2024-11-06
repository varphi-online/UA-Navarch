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
use std::time::Duration;

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

                let handle = tokio::spawn(async move {
                    let start = if first_iter
                        .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
                        .is_ok()
                    {
                        match start_idx.clone() {
                            Some(idx) => {
                                println!("(Starting from the {}th course)", idx);
                                idx.as_str().parse::<usize>().unwrap_or(0_usize)
                            }
                            None => 0,
                        }
                    } else {
                        0
                    };

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
                            .unwrap(),
                    );

                    spinner.set_message(format!("Expanding departments for letter {}", letter));
                    spinner.enable_steady_tick(Duration::from_millis(50));

                    let expand_text = client.expand_departments(letter).await;
                    spinner.finish_and_clear();

                    let course_ids =
                        get_highest_course_number(expand_text.as_str()).unwrap() as usize;

                    let progress_bar = multi_progress.add(ProgressBar::new(course_ids as u64));
                    progress_bar.set_style(
                    ProgressStyle::with_template(
                        "{msg}\n{wide_bar} [{pos}/{len}]\nElapsed: [{elapsed_precise}] | ETA: [{eta_precise}]",
                    )
                    .unwrap()
                );
                    progress_bar.set_message(format!(
                        "Processing letter: {}..{}",
                        letter, client.state_num
                    ));

                    for id in start..course_ids {
                        create_dir_all(format!("{}/course_{id}/", &letter_path)).unwrap();

                        client
                            .extract_class_details(
                                id.to_string().as_str(),
                                format!("{}/course_{id}/", &letter_path),
                            )
                            .await;

                        progress_bar.inc(1);
                    }
                    progress_bar
                        .finish_with_message(format!("Completed processing letter: {}", letter));
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
    let re = Regex::new(r"CRSE_TITLE\$\d+").unwrap();

    re.find_iter(text)
        .filter_map(|m| m.as_str().split('$').last()?.parse::<u32>().ok())
        .max()
}
