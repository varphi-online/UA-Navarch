use super::database::*;
use html5ever;
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

pub fn update_database(letters: &str, db_path: &str, thread_count: usize) {
    let connection = database_init(db_path);

    let letters: Vec<char> = letters.chars().collect();
    for chunk in letters.chunks(thread_count) {
        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        for &letter in chunk {
            for _ in 0..letters.len() {
                let conncton = Arc::clone(&connection);
                let handle: JoinHandle<()> = spawn(|| ());
                handles.push(handle);
            }
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
