// this whole thing is pointless, there is a weedmaps http api that can give json. maybe i will switch over to that as a backend sometime in the future
// for now i'm going to keep improving this and have it output useful information for buying cheap weed


use scraper::Html;
use scraper::Selector;
use std::fmt;
use serde::{Serialize, Deserialize};



#[derive(Debug)]
struct ScrapingError;



#[derive(Serialize, Deserialize)]
enum EdibleType {
    Gummies,
    Chocolate,
    Brownies,
    Other,
}

#[derive(Serialize, Deserialize)]
enum ListingKind {
    Flower(Option<Strain>),
    Cart(bool), //whether or not the cart is live resin
    Edible(EdibleType),
    Other(String),
}

#[derive(Serialize, Deserialize)]
enum Strain {
    Sativa,
    Hybrid,
    Indica
}

#[derive(Serialize, Deserialize)]
struct Listing {
    name: String,
    brand: Option<String>,
    price: String,
    med: bool,
    weight: Option<f32>,
    purity: Option<f32>,
    kind: ListingKind,
}

impl fmt::Display for Listing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, Brand: {:?}, Price: {}", self.name, self.brand, self.price)
    }
}

impl Default for Listing {
    fn default() -> Listing {
        Listing {
            name: String::from("Scraping error - check logs!"),
            brand: None,
            price: String::from("$0"),
            kind: ListingKind::Other(String::from("Scraping error - check logs!"))
        }
    }
}


fn main() {
    println!("Hello, world!");

    let list = scrape_page("https://weedmaps.com/dispensaries/sticky-ypsi?sortBy=name&sortOrder=asc").unwrap();

    for listing in list {
        println!("{}", serde_json::to_string(&listing).unwrap());
    }
}


fn scrape_page(url: &str) -> Result<Vec<Listing>, reqwest::Error> {
    //download the entire page
    let body = reqwest::blocking::get(url)?.text()?;
    //println!("{}",body);

    //scrape for product entries
    let body = Html::parse_document(&body);
    let selector = Selector::parse("[data-test-id=menu-item-list-item]").unwrap();

    //extract data
    let mut list = Vec::new(); //vector of product listings
    for element in body.select(&selector) { //for each listing element
        list.push(scrape_entry(&element).unwrap_or_default()); //scrape and add to list
    }

    Ok(list)
}


fn scrape_entry(element: &scraper::ElementRef) -> Result<Listing, ScrapingError> {
    let name_selector = Selector::parse("[data-testid=menu-item-title]").unwrap(); // for *some* reason, some elements are "data-test-id" while others are "data-testid" :|
    let brand_selector = Selector::parse("[data-testid=menu-item-brand]").unwrap();
    let price_selector = Selector::parse("div").unwrap(); //yeah, i know
    let med_selector;
    let category_selector = Selector::parse("[data-testid=menu-item-category]").unwrap();
    let purity_selector = Selector::parse("[data-testid=menu-item-metrics]").unwrap();

    Ok(Listing { 
        name: element.select(&name_selector).next().ok_or(ScrapingError)?.inner_html(),
        brand: match element.select(&brand_selector).next() {
            Some(elm) => Some(elm.inner_html()),
            None => None,
        }, //this should be one line
        price: element.select(&price_selector).find(|&elm| elm.inner_html().starts_with('$') ).ok_or(ScrapingError)?.inner_html(), //yeahhhhh
        med: false, //todo: make this do something
        weight: Option<f32>,
        purity: Option<f32>,
        kind: find_kind(element)
    })
}

fn find_kind(element: &scraper::ElementRef) -> ListingKind {



    ListingKind::Other(String::from("Some sort of error"))
}
