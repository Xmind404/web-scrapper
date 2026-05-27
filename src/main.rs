fn dork(arg: &str, inurl: bool, intext: bool, site: bool, intitle: bool) -> String {

    // ========< Preparation for Url >========
    let mut parts = String::new();

    if inurl {parts.push_str(format!("inurl:{} ", arg).as_str());}
    if intext {parts.push_str(format!("intext: {} ", arg).as_str());}
    if site {parts.push_str(format!("site:{} ", arg).as_str());}
    if intitle {parts.push_str(format!("intitle:{} ", arg).as_str().trim_start_matches("http://").trim_start_matches("https://"));}

    // ========< Url generation >========
    let search = parts.as_str();
    let safe_search = urlencoding::encode(search).to_string();
    let url = format!("https://google.com/search?q={}", safe_search);

    // ========< Downloading site as single line >========
    reqwest::blocking::get(&url)
        .and_then(|res| res.text())
        .unwrap_or_else(|_| "Network error".to_string())
}



fn main() {
    println!("{}", dork("python.org", true, true,true,true));
}