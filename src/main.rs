use regex::Regex;
use std::collections::HashSet;
fn dork(arg: &str, inurl: bool, intext: bool, site: bool, intitle: bool) -> String {
    let configurations = [
        (inurl, "inurl"),
        (intext, "intext"),
        (site, "site"),
        (intitle, "intitle"),
    ];


    // ========< HashSet for unique links >========
    let mut unique_links = HashSet::new();
    let mut ordered_links = Vec::new();


    // ========< Spoofing Client >========
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();


    // ========< Regex for extracting links from html >========
    let regex = Regex::new(r#"uddg=([^&"]+)"#).unwrap();

    // ========< Help variable for detecting JavaScript Challanges >========
    let mut js_block_detected= false;

    // ========< Checking every operator separately
    for(is_enabled, operator_name) in configurations {

        // ========<Cleaning start for site operator >========
        let clean_arg = if operator_name == "site" {
            arg.trim_start_matches("http://").trim_start_matches("https://")
        }else{
            arg
        };

        // ========< Url generation >========
        let search = format!("{}:{}", operator_name, clean_arg);
        let safe_search = urlencoding::encode(&search).to_string();
        let url = format!("https://duckduckgo.com/html/?q={}", safe_search);

        // ========< Downloading site as single line >========
        let html_content = client.get(&url).send().and_then(|res| res.text()).unwrap_or_else(|_| "Network error".to_string());

        // ========< Detect demanding JavaScript >========
        if html_content.contains("enablejs") || html_content.contains("JavaScript") {
            js_block_detected = true;
        }

        // ========< Decoding link from extracted links by Regex >========
        for cap in regex.captures_iter(html_content.as_str()) {
            if let Some(link) = cap.get(1) {
                let found_link = link.as_str();

                if let Ok(decoded) = urlencoding::decode(found_link) {
                    let clean_link = decoded.into_owned();

                    // ========< Adding cleaned link to links Vector >========
                    if clean_link.starts_with("http") && !clean_link.contains("duckduckgo.com")  {
                        if unique_links.insert(clean_link.clone()) {
                            ordered_links.push(clean_link);
                        }
                    }
                }
            }
        }
    }
    // ========< Return links or Error >========
    if ordered_links.is_empty() {
        if js_block_detected {
            "DuckDuckGo demand JavaScript verification".to_string()
        } else {
            "No search results found".to_string()
        }
    } else {
        ordered_links.join("\n")
    }
}



fn main() {
    println!("{}", dork("python.org", false, true,false,false));
}