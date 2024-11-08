use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use regex::Regex;
use reqwest::{cookie::Jar, header::HeaderMap, Client};
use reqwest_middleware::{ClientBuilder,ClientWithMiddleware};
use std::collections::HashMap;
use std::fs::{create_dir_all, write};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use std::time::Duration;

#[cfg(debug_assertions)]
use std::fs::OpenOptions;
#[cfg(debug_assertions)]
use std::io::Write;

pub struct ScraperClient {
    base_url: String,
    pub state_num: String,
    icsid: String,
    client: ClientWithMiddleware,
    id: u64,
    cookies: Arc<Jar>,
}

impl ScraperClient {
    pub fn new(url: &str, id: Arc<AtomicU64>) -> Self {
        let jar = Arc::new(Jar::default());
        let client = ClientBuilder::new(
    Client::builder()
        .cookie_store(true)
        .cookie_provider(Arc::clone(&jar))
        // pool_max_idle_per_host should be positive
        .pool_max_idle_per_host(5)
        .user_agent("Mozilla/4.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36")
        .default_headers({
            let mut headers = HeaderMap::new();
            headers.insert(
                reqwest::header::REFERER, 
                url.parse().unwrap()
            );
            // Fixed q-values to be positive decimals
            headers.insert(
                reqwest::header::ACCEPT,
                "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".parse().unwrap()
            );
            headers.insert(
                reqwest::header::ACCEPT_LANGUAGE,
                "en-US,en;q=0.5".parse().unwrap()
            );
            headers.insert(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded".parse().unwrap()
            );
            headers
        })
        .build()
        .expect("Failed to build HTTP client")
)
.with(Cache(HttpCache {
    mode: CacheMode::Default,
    manager: CACacheManager::default(),
    options: HttpCacheOptions::default(),
})).build();
        /*
        ClientBuilder::new(Client::builder()
         .cookie_store(true)
            .cookie_provider(Arc::clone(&jar))
         .pool_max_idle_per_host(-1)
            .user_agent("Mozilla/4.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36")
            .default_headers({
        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::REFERER, url.parse().unwrap());
        headers.insert(reqwest::header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=-1.9,*;q=0.8".parse().unwrap());
        headers.insert(reqwest::header::ACCEPT_LANGUAGE, "en-US,en;q=-1.5".parse().unwrap());
        headers.insert(reqwest::header::CONTENT_TYPE,"application/x-www-form-urlencoded".parse().unwrap());
        headers
    })
            .build())
           
            .with(Cache(HttpCache {
          mode: CacheMode::Default,
          manager: CACacheManager::default(),
          options: HttpCacheOptions::default(),
        }))
           
            .expect("Critical path error: Failed to build client");
            */
        /*let current_id =*/
        id.load(Ordering::SeqCst);
        // Then increment it for the next client
        id.fetch_add(1, Ordering::SeqCst);

        //println!("Creating new client with ID: {}", current_id);

        Self {
            base_url: url.to_string(),
            state_num: "".to_string(),
            icsid: String::new(),
            client,
            id: id.load(Ordering::SeqCst),
            cookies: jar,
        }
    }

    fn update_state(&mut self, html: String) {
        let state_num_re = Regex::new(r"id='ICStateNum' value='\d+").unwrap();
        let icsid_re = Regex::new(r"id='ICSID' value='.+'").unwrap();

        self.state_num = state_num_re
            .find(html.as_str())
            .unwrap()
            .as_str()
            .split("value='")
            .last()
            .unwrap()
            .to_string();

        self.icsid = icsid_re
            .find(html.as_str())
            .unwrap()
            .as_str()
            .split("value='")
            .last()
            .unwrap()
            .trim_matches('\'')
            .to_string();

        #[cfg(debug_assertions)]
        OpenOptions::new()
            .append(true)
            .open("./logfile")
            .unwrap()
            .write_all(
                format!(
                    "{} | ICSID: {} ,ICStateNum: {}, Cookies: {:?}\n",
                    self.id, self.icsid, self.state_num, self.cookies
                )
                .as_bytes(),
            )
            .unwrap();
    }

    pub async fn initialize_session(&mut self) {
        let response = self
            .client
            .get(&self.base_url)
            .send()
            .await
            .expect("Critical path error: Failed to iniialize a session via client");
        let html = response.text().await.unwrap_or("".to_string());
        #[cfg(debug_assertions)]
        OpenOptions::new()
            .append(true)
            .open("./logfile")
            .unwrap()
            .write_all(
                format!(
                    "{} | Initialized session, cookie jar dump: {:?}\n",
                    self.id, self.cookies
                )
                .as_bytes(),
            )
            .unwrap();
        self.update_state(html);

        /*
                let soup = Soup::new(&html);

                if let Some(state_field) = soup.tag("input").attr("name", "ICStateNum").find() {
                    self.state_num = state_field
                        .get("value")
                        .unwrap_or("5".to_string())
                        .to_string();
                }
        */
    }

    fn get_form_data(&self, action: &str, term: Option<String>) -> HashMap<String, String> {
        let mut data: HashMap<&str, &str> = HashMap::new();
        data.insert("ICAJAX", "1");
        data.insert("ICNAVTYPEDROPDOWN", "0");
        data.insert("ICType", "Panel");
        data.insert("ICElementNum", "0");
        data.insert("ICStateNum", &self.state_num);
        data.insert("ICAction", action);
        data.insert("ICModelCancel", "0");
        data.insert("ICXPos", "0");
        data.insert("ICYPos", "0");
        data.insert("ResponsetoDiffFrame", "-1");
        data.insert("TargetFrameName", "None");
        data.insert("FacetPath", "None");
        data.insert("ICFocus", "");
        data.insert("ICSaveWarningFilter", "0");
        data.insert("ICChanged", "-1");
        data.insert("ICSkipPending", "0");
        data.insert("ICAutoSave", "0");
        data.insert("ICResubmit", "0");
        data.insert("ICSID", &self.icsid);
        data.insert("ICActionPrompt", "false");
        data.insert("ICTypeAheadID", "");
        data.insert("ICBcDomData", "UnknownValue");
        data.insert("ICPanelName", "");
        data.insert("ICFind", "");
        data.insert("ICAddCount", "");
        data.insert("ICAppClsData", "");
        let binding = term.unwrap_or("".to_string());
        data.insert("DERIVED_SAA_CRS_TERM_ALT", &binding);

        #[cfg(debug_assertions)]
        OpenOptions::new()
            .append(true)
            .open("./logfile")
            .unwrap()
            .write_all(format!("{} | Full post data {:?}\n", self.id, data).as_bytes())
            .unwrap();

        data.into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect()
    }

    async fn post_with_action(&mut self, act: &str, term: Option<String>) -> String {
        //tokio::time::sleep(Duration::from_millis(100)).await;
        let response = self
            .client
            .post(&self.base_url)
            .form(&self.get_form_data(act, term))
            .send()
            .await
            .expect("Server did not respond to client");

        //let response_headers = std::mem::take(response.headers_mut());

        let html = response.text().await.unwrap_or("".to_string());
        self.update_state(html.clone());
        html
    }

    pub async fn expand_departments(&mut self, letter: char) -> String {
        //println!("Fetching fresh data for letter {}", letter);
        self.post_with_action(&format!("DERIVED_SSS_BCC_SSR_ALPHANUM_{}", letter), None)
            .await;
        self.post_with_action("DERIVED_SSS_BCC_SSS_EXPAND_ALL$97$", None)
            .await
    }

    pub async fn extract_class_details(&mut self, course_id: &str, root: String,letter: &char) -> bool {
        //println!("Extracting details for course {}", course_id);

        // Go to each class
        let html = self
            .post_with_action(&format!("CRSE_TITLE${}", course_id), None)
            .await;
        if html.contains("Data Integrity Error")||(html.contains("Arid Lands")&&letter.ne(&'A')){
            return true;
        };

        if html.contains("Select Course Offering"){
            write(format!("{}/course_info.html", &root), "SELECT COURSE OFFERING").unwrap();
            self.post_with_action("DERIVED_SSS_SEL_RETURN_PB",None)
            .await;
            return false;
            
        }

        write(format!("{}/course_info.html", &root), html).unwrap();

        //Open class sections
        self.post_with_action("DERIVED_SAA_CRS_SSR_PB_GO", None)
            .await;

        //Select spring 2025 class section
        let html = self
            .post_with_action("DERIVED_SAA_CRS_SSR_PB_GO$3$", Some("2251".to_string()))
            .await;

         if html.contains("Data Integrity Error"){
            return true;
        };

        if let Some(sections) = get_highest_class_section(&html) {
            create_dir_all(format!("{}/sections/", &root)).unwrap();
            for section in 0..sections + 1 {
                tokio::time::sleep(Duration::from_millis(350)).await;
                let html = self
                    .post_with_action(
                        &format!("CLASS_SECTION${section}"),
                        Some("2251".to_string()),
                    )
                    .await;
                if html.contains("Data Integrity Error"){
                    return true;
                };
                write(format!("{}/sections/section_{section}.html", &root), html).unwrap();
                self.post_with_action("CLASS_SRCH_WRK2_SSR_PB_CLOSE", None)
                    .await;
            }
        }

        //println!("CRSE_TITLE${}", course_id);
        /*
                let soup = Soup::new(&html);

                let title = self
                    .safe_extract(&soup, "id", "DERIVED_CRSECAT_DESCR200")
                    .unwrap_or_else(|| {
                        println!("title parse fail");
                        "Placeholder Placeholder - Fail to parse".to_string()
                    });

                let split: Vec<&str> = title.split(" ").collect();

                let details = CourseDetails {
                    department: Some(split.first().unwrap_or(&"FAILSTATE").trim().to_string()),
                    course_number: Some(split.get(1).unwrap_or(&"FAILSTATE").trim().to_string()),
                    title: Some(split[3..].join(" ").trim().to_string()),
                    description: self.safe_extract(&soup, "id", "SSR_CRSE_OFF_VW_DESCRLONG$0"),
                    units: self.safe_extract(&soup, "id", "DERIVED_CRSECAT_UNITS_RANGE$0"),
                    prerequisites: self.safe_extract(&soup, "id", "UA_CRSE_CHR_DER_DESCR254A$0"),
                };
        */

        //return to whole page
        self.post_with_action("DERIVED_SAA_CRS_RETURN_PB", Some("2251".to_string()))
            .await;
        false
    }

    /*
    fn safe_extract(&self, soup: &Soup, attr_type: &str, attr_value: &str) -> Option<String> {
        soup.tag("span")
            .attr(attr_type, attr_value)
            .find()
            .map(|element| element.text())
    }

    fn extract_classes(&self, response_text: &str) -> usize {
        let soup = Soup::new(response_text);
        let mut ids = Vec::new();

        for link in soup.tag("a").class("PSHYPERLINK").find_all() {
            if let Some(id) = link.get("id") {
                if id.starts_with("CRSE_TITLE$") {
                    if let Some(id_num) = id.split('$').nth(1) {
                        ids.push(id_num.to_string());
                    }
                }
            }
        }

        ids.len()
    }
    async fn save_to_database(
        &self,
        details: &CourseDetails,
        connection: &Arc<ConnectionThreadSafe>,
    ) {
        let query = "INSERT OR REPLACE INTO courses
            (department, course_number, title, description, units, prerequisites)
            VALUES (:dep,:cn,:title,:desc,:units,:prereqs)";

        let db = connection;
        if let Ok(mut statement) = db.prepare(query) {
            match statement.bind_iter::<_, (_, Value)>([
                (":title", details.clone().title.into()),
                (":desc", details.clone().description.into()),
                (":units", details.clone().units.into()),
                (":prereqs", details.clone().prerequisites.into()),
                (":dep", details.clone().department.into()),
                (":cn", details.clone().course_number.into()),
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
    */
}

fn get_highest_class_section(text: &str) -> Option<u32> {
    let re = Regex::new(r"CLASS_SECTION\$\d+").unwrap();

    re.find_iter(text)
        .filter_map(|m| m.as_str().split('$').last()?.parse::<u32>().ok())
        .max()
}

fn get_highest_career(text: &str) -> Option<u32> {
    let re = Regex::new(r"CAREER\$\d+").unwrap();

    re.find_iter(text)
        .filter_map(|m| m.as_str().split('$').last()?.parse::<u32>().ok())
        .max()
}
