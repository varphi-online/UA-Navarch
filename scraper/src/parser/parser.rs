use super::database::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use regex::Regex;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

pub fn update_database(letters: &str, cache_path: &str, db_path: &str, thread_count: usize) {
    let connection = database_init(db_path);

    let letters: Vec<char> = letters.chars().collect();
    let multi_progress = Arc::new(MultiProgress::new());
    for chunk in letters.chunks(thread_count) {
        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        for &letter in chunk {
            let connection = Arc::clone(&connection);
            let multi_progress = Arc::clone(&multi_progress);
            let root = cache_path.to_owned();

            // The actual program, inside a thread

            let handle: JoinHandle<()> = spawn(move || {
                let letter_path = format!("{}{}_deps", root, letter);
                //println!("folder: {letter_path}");
                let course_count = std::fs::read_dir(&letter_path)
                    .unwrap()
                    .filter_map(|entry| entry.ok())
                    .count();
                let progress_bar = multi_progress.add(ProgressBar::new(course_count as u64));
                for course_num in 0..course_count {
                    progress_bar.set_style(ProgressStyle::with_template("Elapsed: [{elapsed_precise}] | ETA: [{eta_precise}] | {msg}\n{wide_bar} [{pos}/{len}]").unwrap());
                    let mut course: Option<Course> = None;
                    let mut sections: Vec<Option<Section>> = Vec::new();
                    //println!("trying: {}/course_{course_num}", &letter_path);
                    if let Ok(read_dir) =
                        std::fs::read_dir(format!("{}/course_{course_num}", &letter_path))
                    {
                        //println!("\tSubdir: {:?}", read_dir);
                        for item in read_dir {
                            let item = item.unwrap().path();
                            if item.is_file() {
                                course = parse_course(&item);
                            } else if let Ok(sub_dir) = std::fs::read_dir(item) {
                                for sub_item in sub_dir {
                                    sections.push(parse_section(&sub_item.unwrap().path()));
                                }
                            }
                        }
                    }
                    if let Some(course) = course.as_ref() {
                        db_course_write(course, &Arc::clone(&connection));
                    }

                    for item in sections.into_iter() {
                        //println!("{:?}", item);
                        if let Some(section) = item {
                            //println!("{:?}", section);
                            db_section_write(&section, &Arc::clone(&connection));
                        };
                    }

                    progress_bar.set_message(format!(
                        "Processing letter: {} - {}",
                        letter,
                        &course.unwrap_or_default().title.unwrap_or_default()
                    ));
                    progress_bar.inc(1);
                }
                progress_bar.finish_and_clear();
            });

            //

            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
}

fn parse_course(path: &std::path::PathBuf) -> Option<Course> {
    if let Ok(html) = std::fs::read_to_string(path) {
        let title = find_by_id(&html, "DERIVED_CRSECAT_DESCR200").unwrap_or_else(|| {
            //println!("title parse fail");
            "Placeholder Placeholder - Fail to parse".to_string()
        });
        let attrs =
            find_by_id(&html, r"DERIVED_CRSECAT_SSR_CRSE_ATTR_LONG\$0").unwrap_or("".to_string());
        let split: Vec<&str> = title.split(" ").collect();

        let (dept, cn) = (
            split
                .first()
                .unwrap_or(&"FAILSTATE")
                .replace("&nbsp;", "")
                .trim()
                .to_string(),
            split.get(1).unwrap_or(&"FAILSTATE").trim().to_string(),
        );

        Some(Course {
            hash: Some(calculate_hash(&format!("{dept}{cn}")) as i64),
            department: Some(dept),
            course_number: Some(cn),
            title: Some(split[3..].join(" ").trim().to_string()),
            description: find_by_id(&html, r"SSR_CRSE_OFF_VW_DESCRLONG\$0"),
            units: find_by_id(&html, r"DERIVED_CRSECAT_UNITS_RANGE\$0"),
            prerequisites: find_by_id(&html, r"UA_CRSE_CHR_DER_DESCR254A\$0"),
            requirements: find_by_id(&html, r"DERIVED_CRSECAT_DESCR254A\$0"),
            equivalences: find_by_id(&html, r"UA_CRSE_CHR_DER_DESCR254_1\$0"),
            //Requirements - DERIVED_CRSECAT_DESCR254A$0
            //Equivalances - UA_CRSE_CHR_DER_DESCR254_1$0
            attr_ge_bc: Some(
                attrs
                    .to_lowercase()
                    .contains("building connections")
                    .to_string(),
            ),
            attr_ge_art: Some(attrs.to_lowercase().contains("artist").to_string()),
            attr_ge_hum: Some(attrs.to_lowercase().contains("humanist").to_string()),
            attr_ge_ns: Some(
                attrs
                    .to_lowercase()
                    .contains("natural scientist")
                    .to_string(),
            ),
            attr_ge_ss: Some(
                attrs
                    .to_lowercase()
                    .contains("social scientist")
                    .to_string(),
            ),
            attr_ge_ec: Some(attrs.to_lowercase().contains("entry course").to_string()),
            attr_ge_xc: Some(attrs.to_lowercase().contains("exit course").to_string()),
            attrs: Some(attrs),
        })
    } else {
        None
    }
}

fn parse_section(path: &std::path::PathBuf) -> Option<Section> {
    if let Ok(html) = std::fs::read_to_string(path) {
        let title = find_by_id(&html, "DERIVED_CLSRCH_DESCR200").unwrap_or_else(|| {
            //println!("title parse fail");
            "Placeholder Placeholder - Fail to parse".to_string()
        });
        let extra = find_by_id(&html, "DERIVED_CLSRCH_SSS_PAGE_KEYDESCR").unwrap_or_else(|| {
            //println!("extra parse fail - DERIVED_CLSRCH_SSS_PAGE_KEYDESCR");
            "Placeholder | Placeholder | Placeholder".to_string()
        });
        let date = find_by_id(&html, r"MTG_DATE\$0").unwrap_or_else(|| {
            //println!("date parse fail");
            "Placeholder - Placeholder".to_string()
        });
        let meeting = find_by_id(&html, r"MTG_SCHED\$0").unwrap_or_else(|| {
            //println!("meeting parse fail - MTG_SCHED$0");
            "TBD TBD - TBD".to_string()
        });
        let t_split: Vec<&str> = title.split_whitespace().collect();
        let e_split: Vec<&str> = extra.split("|").collect();
        let d_split: Vec<&str> = date.split("-").collect();
        let m_split: Vec<&str> = meeting.split_whitespace().collect();

        let (dept, cn) = (
            t_split
                .first()
                .unwrap_or(&"FAILSTATE")
                .replace("&nbsp;", "")
                .trim()
                .to_string(),
            t_split.get(1).unwrap_or(&"FAILSTATE").trim().to_string(),
        );

        //println!("{:?}{:?}{:?}{:?}", t_split, e_split, d_split, m_split);
        Some(Section {
            hash: Some(calculate_hash(&format!("{dept}{cn}")) as i64),
            department: Some(dept),
            course_number: Some(cn),
            class_number: find_by_id(&html, "SSR_CLS_DTL_WRK_CLASS_NBR"),
            section_number: Some(
                t_split
                    .get(3)
                    .unwrap_or(&"FAILSTATE")
                    .replace("&nbsp;", "")
                    .trim()
                    .to_string(),
            ),
            term: Some(e_split.get(1).unwrap_or(&"FAILSTATE").trim().to_string()),
            status: find_by_id(&html, "SSR_CLS_DTL_WRK_SSR_DESCRSHORT"),
            session: find_by_id(&html, r"PSXLATITEM_XLATLONGNAME\$31\$"),
            class_components: find_by_id(&html, r"SSR_CLS_DTL_WRK_SSR_COMPONENT_LONG\$273\$").map(
                |comp| {
                    comp.replace(
                        "&lt;table class=&quot;PSTEXT&quot;&gt;&lt;tr&gt;&lt;td&gt;",
                        "",
                    )
                    .replace("&lt;/td&gt;&lt;/tr&gt;&lt;/table&gt;", "")
                },
            ),
            instruction_mode: find_by_id(&html, "INSTRUCT_MODE_DESCR"),
            class_type: Some(e_split.get(2).unwrap_or(&"FAILSTATE").trim().to_string()),
            academic_career: find_by_id(&html, "PSXLATITEM_XLATLONGNAME"),
            start_date: Some(d_split.get(0).unwrap_or(&"FAILSTATE").trim().to_string()),
            end_date: Some(d_split.get(1).unwrap_or(&"FAILSTATE").trim().to_string()),
            grading: find_by_id(&html, "GRADE_BASIS_TBL_DESCRFORMAL"),
            location: find_by_id(&html, "CAMPUS_LOC_VW_DESCR"),
            campus: find_by_id(&html, "CAMPUS_TBL_DESCR"),
            monday: Some(
                m_split
                    .get(0)
                    .unwrap_or(&"FALSE")
                    .contains("Mo")
                    .to_string(),
            ),
            tuesday: Some(
                m_split
                    .get(0)
                    .unwrap_or(&"FALSE")
                    .contains("Tu")
                    .to_string(),
            ),
            wednesday: Some(
                m_split
                    .get(0)
                    .unwrap_or(&"FALSE")
                    .contains("We")
                    .to_string(),
            ),
            thursday: Some(
                m_split
                    .get(0)
                    .unwrap_or(&"FALSE")
                    .contains("Th")
                    .to_string(),
            ),
            friday: Some(
                m_split
                    .get(0)
                    .unwrap_or(&"FALSE")
                    .contains("Fr")
                    .to_string(),
            ),
            start_time: Some(convert_time_format(m_split.get(1).unwrap_or(&"TBD").trim())),
            end_time: Some(convert_time_format(m_split.get(3).unwrap_or(&"TBD").trim())),
            instructor: find_by_id(&html, r"MTG_INSTR\$0"),
            class_capacity: find_by_id(&html, "SSR_CLS_DTL_WRK_ENRL_CAP"),
            enrollment_total: find_by_id(&html, "SSR_CLS_DTL_WRK_ENRL_TOT"),
            available_seats: find_by_id(&html, "SSR_CLS_DTL_WRK_AVAILABLE_SEATS"),
        })
    } else {
        None
    }
}
// TODO: LAZY STATIC
fn find_by_id(html: &str, id: &str) -> Option<String> {
    let scanner = format!(r#"id=['|"]{}['|"][.|\s]*>(?P<first>.*)<"#, id);
    let scan = Regex::new(&scanner).unwrap();
    scan.captures(html)
        .and_then(|caps| caps.name("first"))
        .map(|m| m.as_str().to_string())
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn convert_time_format(time_str: &str) -> String {
    if time_str == "TBD" {
        return time_str.to_string();
    };
    let time = &time_str[..time_str.len() - 2];
    let meridiem = &time_str[time_str.len() - 2..];

    let mut parts = time.split(':');
    let mut hours: u32 = parts.next().unwrap().parse().unwrap();
    let minutes: u32 = parts.next().unwrap().parse().unwrap();

    if meridiem == "PM" && hours != 12 {
        hours += 12;
    } else if meridiem == "AM" && hours == 12 {
        hours = 0;
    }

    format!("{:02}:{:02}", hours, minutes)
}

#[derive(Clone, Default, Debug)]
pub struct Course {
    //pub course_id: Option<String>,
    pub hash: Option<i64>,
    pub department: Option<String>,
    pub course_number: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub units: Option<String>,
    pub prerequisites: Option<String>,
    pub requirements: Option<String>,
    pub equivalences: Option<String>,
    pub attr_ge_bc: Option<String>,
    pub attr_ge_art: Option<String>,
    pub attr_ge_hum: Option<String>,
    pub attr_ge_ns: Option<String>,
    pub attr_ge_ss: Option<String>,
    pub attr_ge_ec: Option<String>,
    pub attr_ge_xc: Option<String>,
    pub attrs: Option<String>,
}

#[derive(Clone, Default, Debug)]
pub struct Section {
    pub hash: Option<i64>,
    pub class_number: Option<String>,
    pub department: Option<String>,
    pub course_number: Option<String>,
    pub section_number: Option<String>,
    pub term: Option<String>,
    pub status: Option<String>,
    pub session: Option<String>,
    pub class_components: Option<String>,
    pub instruction_mode: Option<String>,
    pub class_type: Option<String>,
    pub academic_career: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub grading: Option<String>,
    pub location: Option<String>,
    pub campus: Option<String>,
    pub monday: Option<String>,
    pub tuesday: Option<String>,
    pub wednesday: Option<String>,
    pub thursday: Option<String>,
    pub friday: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub instructor: Option<String>,
    pub class_capacity: Option<String>,
    pub enrollment_total: Option<String>,
    pub available_seats: Option<String>,
}
