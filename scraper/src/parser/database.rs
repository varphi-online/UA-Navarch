use sqlite::ConnectionThreadSafe;
use std::collections::HashMap;
use std::sync::Arc;

pub fn database_init(db_path: &str) -> Arc<ConnectionThreadSafe> {
    let conn = Arc::new(sqlite::Connection::open_thread_safe(&db_path).unwrap());
    conn.execute(
        "CREATE TABLE IF NOT EXISTS courses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            department TEXT,
            course_number TEXT,
            title TEXT,
            description TEXT,
            units TEXT,
            prerequisites TEXT
            ); 
         CREATE TABLE IF NOT EXISTS spring25 (
            department TEXT,
            course_number TEXT,
            section TEXT,
            
        )",
    )
    .unwrap();
    Arc::clone(&conn)
}

async fn save_to_database(
    details: &HashMap<String, Option<String>>,
    connection: &Arc<ConnectionThreadSafe>,
) {
    let query = "INSERT OR REPLACE INTO courses
            (department, course_number, title, description, units, prerequisites)
            VALUES (:dep,:cn,:title,:desc,:units,:prereqs)";

    let db = connection;
    if let Ok(mut statement) = db.prepare(query) {
        match statement.bind_iter::<_, (_, sqlite::Value)>([
            (":title", details.get(":title").unwrap().into()),
            (":desc", details.get(":title").into()),
            (":units", details.get(":title").into()),
            (":prereqs", details.get(":title").into()),
            (":dep", details.get(":title").into()),
            (":cn", details.get(":title").into()),
        ]) {
            Ok(()) => {
                let _ = statement.next().unwrap_or_else(|_| {
                    println!(
                        "Failed to advance Database state to prepared query for course: {:?} {:?}",
                        details.title, details.course_number
                    );
                    sqlite::State::Done
                });
            }
            Err(_) => {
                println!(
                    "Failed to bind some course data to a prepared query for course: {:?} {:?}",
                    details.title, details.course_number
                )
            }
        }
    } else {
        println!(
            "Failed to prepare a database query for course: {:?} {:?}",
            details.title, details.course_number
        )
    }
}
