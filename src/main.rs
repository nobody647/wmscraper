use scraper::Html;
use scraper::Selector;
use std::fmt;

struct Listing {
    name: String,
    brand: Option<String>,
    price: String,
}

impl fmt::Display for Listing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, Brand: {:?}, Price: {}", self.name, self.brand, self.price)
    }
}


fn main() {
    println!("Hello, world!");

    let list = scrape_page("https://weedmaps.com/dispensaries/sticky-ypsi?sortBy=name&sortOrder=asc").unwrap();

    for listing in list {
        println!("{}", listing);
    }
}


fn scrape_page(url: &str) -> Result<Vec<Listing>, reqwest::Error> {
    //download the entire page
    let body = reqwest::blocking::get(url)?.text()?;
    println!("{}",body);

    //scrape for product entries
    let body = Html::parse_document(&body);
    let selector = Selector::parse("[data-test-id=menu-item-list-item]").unwrap();

    //extract data
    let mut list = Vec::new();
    for element in body.select(&selector) {
        println!("Element with id {:?} found", element.value().attr("data-test-id"));
        list.push(scrape_entry(&element));
    }

    Ok(list)
}


fn scrape_entry(element: &scraper::ElementRef) -> Listing {
    let name_selector = Selector::parse("[data-testid=menu-item-title]").unwrap(); // for *some* reason, some elements are "data-test-id" while others are "data-testid" :|
    let brand_selector = Selector::parse("[data-testid=menu-item-brand]").unwrap();
    let price_selector = Selector::parse("div").unwrap(); //yeah, i know

    Listing { 
        name: element.select(&name_selector).next().unwrap().inner_html(),
        brand: match element.select(&brand_selector).next() {
            Some(elm) => Some(elm.inner_html()),
            None => None,
        }, //this should be one line
        price: element.select(&price_selector).find(|&elm| elm.inner_html().starts_with('$')).unwrap().inner_html(),
    }
}
