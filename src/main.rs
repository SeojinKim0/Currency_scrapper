fn main() {
    let mut vec = Vec::new();
    let url = "https://finance.naver.com/marketindex/exchangeDetail.naver?marketindexCd=FX_CADKRW";
    let response = reqwest::blocking::get(url).expect("Can't load url");
    let html_string = response.text().unwrap();

    let document = scraper::Html::parse_document(&html_string);
    let sale_selector = scraper::Selector::parse(".no_today").unwrap();

    for e in document.select(&sale_selector) {
        vec = e.text().collect::<Vec<_>>();
    }

    let filtered_vec: Vec<&str> = vec.iter().filter(|s| !s.trim().is_empty()).map(|s| s.trim()).collect();
    let val = filtered_vec.join("");
    println!("{}", val);
}
