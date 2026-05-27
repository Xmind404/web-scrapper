fn dork(arg: &str, inurl: bool, intext: bool, site: bool, intitle: bool) -> String {
    let mut parts = Vec::new();

    if inurl {
        parts.push(format!("inurl:{}", arg));
    }
    if intext {
        parts.push(format!("intext:{}", arg));
    }
    if site {
        parts.push(format!("site:{}", arg.trim_start_matches("https://").trim_start_matches("http://")));
    }
    if intitle {
        parts.push(format!("intitle:{}", arg));
    }

    let search = parts.join(" ");
    let safe_search = urlencoding::encode(search.as_str()).into_owned();

    let link = format!("https://google.com/search?q={}", safe_search);
    link
}

fn main() {
    println!("{}", dork("python.org", false, true, true, false));
    println!("{}", dork("login", true, false, false, true));
}