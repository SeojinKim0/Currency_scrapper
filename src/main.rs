// use scraper::Html;
// use scraper::Selector;
// use reqwest::*;

fn main() {
    let mut vec = Vec::new();
    let url = "https://finance.naver.com/marketindex/exchangeDetail.naver?marketindexCd=FX_CADKRW";
    let response = reqwest::blocking::get(url).expect("Can't load url");
    let html_string = response.text().unwrap();
    // dbg!(&response);

    let document = scraper::Html::parse_document(&html_string);
    let sale_selector = scraper::Selector::parse(".no_today").unwrap();
    // dbg!(&sale_selector);
    // let currencies = document.select(&sale_selector).map(|x| x.inner_html());
    // currencies.zip(1..10).for_each(|(currency, index)| println!("{}, {}", index, currency));

    for e in document.select(&sale_selector) {
        // dbg!(e);
        vec = e.text().collect::<Vec<_>>();
        // println!("{:?}", story_txt);
        // println!("{}", e);
    }
    let filtered_vec: Vec<&str> = vec.iter().filter(|s| !s.trim().is_empty()).map(|s| s.trim()).collect();

    // println!("{:?}", filtered_vec);
    let val = filtered_vec.join("");
    println!("{}", val);
}
