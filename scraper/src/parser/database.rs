use super::parser::{Course, Section};
use sqlite::ConnectionThreadSafe;
use std::sync::Arc;

pub fn database_init(db_path: &str) -> Arc<ConnectionThreadSafe> {
    let conn = Arc::new(sqlite::Connection::open_thread_safe(db_path).unwrap());
    conn.execute("DROP TABLE courses; DROP TABLE sections");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS courses (
            hash INTEGER UNIQUE,
            department TEXT,
            course_number TEXT,
            title TEXT,
            description TEXT,
            units TEXT,
            prerequisites TEXT,
            requirements TEXT,
            equivalences TEXT,
            building_connections TEXT,
            artist TEXT,
            humanist TEXT,
            natural_scientist TEXT,
            social_scientist TEXT,
            entry_course TEXT,
            exit_course TEXT,
            attributes TEXT
            )",
    )
    .unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sections (
            hash INTEGER,
            course_hash INTEGER,
            class_number TEXT,
            department TEXT,
            course_number TEXT,
            section_number TEXT,
            term TEXT,
            status TEXT,
            session TEXT,
            class_components TEXT,
            instruction_mode TEXT,
            class_type TEXT,
            academic_career TEXT,
            start_date TEXT,
            end_date TEXT,
            grading TEXT,
            location TEXT,
            campus TEXT,
            monday TEXT,
            tuesday TEXT,
            wednesday TEXT,
            thursday TEXT,
            friday TEXT,
            start_time TEXT,
            end_time TEXT,
            instructor TEXT,
            class_capacity TEXT,
            enrollment_total TEXT,
            available_seats TEXT
        )",
    )
    .unwrap();
    Arc::clone(&conn)
}

pub fn db_course_write(details: &Course, connection: &Arc<ConnectionThreadSafe>) {
    let query = "INSERT OR REPLACE INTO courses
            (hash, department, course_number, title, description, units,
            prerequisites, requirements, equivalences, building_connections, artist, humanist, natural_scientist,
            social_scientist, entry_course, exit_course, attributes)
            VALUES (:h,:dep,:cn,:title,:desc,:units,:prereqs,:req,:eq,:bc,:art,:hum,:ns,:ss,:ec,:xc,:attr)";

    let db = connection;
    if let Ok(mut statement) = db.prepare(query) {
        match statement.bind_iter::<_, (_, sqlite::Value)>([
            (":h", details.clone().hash.into()),
            (":title", details.clone().title.into()),
            (":desc", details.clone().description.into()),
            (":units", details.clone().units.into()),
            (":prereqs", details.clone().prerequisites.into()),
            (":req", details.clone().requirements.into()),
            (":eq", details.clone().equivalences.into()),
            (":dep", details.clone().department.into()),
            (":cn", details.clone().course_number.into()),
            (":bc", details.clone().attr_ge_bc.into()),
            (":art", details.clone().attr_ge_art.into()),
            (":hum", details.clone().attr_ge_hum.into()),
            (":ns", details.clone().attr_ge_ns.into()),
            (":ss", details.clone().attr_ge_ss.into()),
            (":ec", details.clone().attr_ge_ec.into()),
            (":xc", details.clone().attr_ge_xc.into()),
            (":attr", details.clone().attrs.into()),
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

pub fn db_section_write(details: &Section, connection: &Arc<ConnectionThreadSafe>) {
    //println!("{:?}", details);
    let query = "INSERT OR REPLACE INTO sections
        (hash, course_hash, class_number, department, course_number, section_number, term, status, session, class_components, instruction_mode, class_type, academic_career, start_date, end_date, grading, location, campus, monday, tuesday, wednesday, thursday, friday, start_time, end_time, instructor, class_capacity, enrollment_total, available_seats)
            VALUES (:h, :course_hash, :class_number, :department, :course_number, :section_number, :term, :status, :session, :class_components, :instruction_mode, :class_type, :academic_career, :start_date, :end_date, :grading, :location, :campus, :monday, :tuesday, :wednesday, :thursday, :friday, :start_time, :end_time, :instructor, :class_capacity, :enrollment_total, :available_seats)";
    let db = connection;
    if let Ok(mut statement) = db.prepare(query) {
        match statement.bind_iter::<_, (_, sqlite::Value)>([
            (":h", details.hash.into()),
            (":course_hash", details.course_hash.into()),
            (":class_number", details.class_number.clone().into()),
            (":department", details.department.clone().into()),
            (":course_number", details.course_number.clone().into()),
            (":section_number", details.section_number.clone().into()),
            (":term", details.term.clone().into()),
            (":status", details.status.clone().into()),
            (":session", details.session.clone().into()),
            (":class_components", details.class_components.clone().into()),
            (":instruction_mode", details.instruction_mode.clone().into()),
            (":class_type", details.class_type.clone().into()),
            (":academic_career", details.academic_career.clone().into()),
            (":start_date", details.start_date.clone().into()),
            (":end_date", details.end_date.clone().into()),
            (":grading", details.grading.clone().into()),
            (":location", details.location.clone().into()),
            (":campus", details.campus.clone().into()),
            (":monday", details.monday.clone().into()),
            (":tuesday", details.tuesday.clone().into()),
            (":wednesday", details.wednesday.clone().into()),
            (":thursday", details.thursday.clone().into()),
            (":friday", details.friday.clone().into()),
            (":start_time", details.start_time.clone().into()),
            (":end_time", details.end_time.clone().into()),
            (":instructor", details.instructor.clone().into()),
            (":class_capacity", details.class_capacity.clone().into()),
            (":enrollment_total", details.enrollment_total.clone().into()),
            (":available_seats", details.available_seats.clone().into()),
        ]) {
            Ok(()) => {
                let _ = statement.next().unwrap_or_else(|_| {
                    println!(
                        "Failed to advance Database state to prepared query for section: {:?} {:?} {:?}",
                        details.department, details.course_number, details.section_number
                    );
                    sqlite::State::Done
                });
            }
            Err(_) => {
                println!(
                    "Failed to bind some section data to a prepared query for section: {:?} {:?} {:?}",
                    details.department, details.course_number, details.section_number
                )
            }
        }
    } else {
        println!(
            "Failed to prepare a database query for section: {:?} {:?} {:?}",
            details.department, details.course_number, details.section_number
        )
    }
}
