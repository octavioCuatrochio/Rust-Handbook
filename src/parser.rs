pub fn parse_url(url: String) -> String {
    let mut searched = "#access_token=";
    let mut index = url.find(searched);
    let start = (index.unwrap_or(0) as i32) + (searched.len() as i32);
    searched = "&token_type";
    index = url.find(searched);
    let end = index.unwrap_or(0) as i32;

    let mut extract: String = String::from("");
    let mut i = 0;

    for c in url.chars() {
        if i >= start && i < end {
            extract.push(c);
        }
        i += 1;
    }
    extract
}
