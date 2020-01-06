use colored;
use colored::Colorize;
use reqwest;
use scraper::{ElementRef, Html, Selector};

const TREND_PREFIX: &str = "https://github.com/trending/";
const GH_PREFIX: &str = "https://github.com/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Scrape and print trending repos for all languages first
    let resp = reqwest::get(TREND_PREFIX).await?;
    let body = resp.text().await?;
    println!(
        "\n{}",
        "Top Trending Github Repos for all languages are:"
            .yellow()
            .bold()
    );
    println!(
        "{}",
        "-----------------------------------------------"
            .yellow()
            .bold()
    );
    scrape(body);
    // Scrape and print trending repos for the varios languages
    // Add languages to this vector to add a languages to scrape
    let langs = vec!["Python", "Rust", "Go"];
    for l in langs {
        println!(
            "\n{} {}",
            "Top Trending Github Repos for".yellow().bold(),
            l.yellow().bold()
        );
        println!("{}", "-----------------------------------".yellow().bold());
        let crawl_url = get_link(l.to_string());
        let lang_resp = reqwest::get(&crawl_url).await?;
        let lang_body = lang_resp.text().await?;
        scrape(lang_body);
    }
    Ok(())
}

// Get the trending url for the specified language
fn get_link(lang: String) -> String {
    format!("{}{}", TREND_PREFIX, lang)
}

// Primary iteratiotor for the scrapes
fn scrape(body: String) {
    let fragment = Html::parse_document(&body);
    let main_selector = Selector::parse("article").unwrap();
    for article in fragment.select(&main_selector) {
        get_repo(article);
        get_description(article);
    }
}

// Scrape for the repo url
fn get_repo(el: ElementRef) {
    let repo_selector = Selector::parse("h1").unwrap();
    // iterate over elements matching the h1 selector
    for r in el.select(&repo_selector) {
        for splits in r.inner_html().split_whitespace() {
            if splits.contains("href") {
                for repo_link in splits.split('\"') {
                    if repo_link.contains("/") {
                        println!("{}{}", GH_PREFIX.blue(), repo_link.blue());
                    }
                }
            }
        }
    }
}

// Scrape for the description
fn get_description(el: ElementRef) {
    let descrip_selector = Selector::parse("p").unwrap();
    // iterate over elements matching the p selector
    for d in el.select(&descrip_selector) {
        let desc = str::replace(&d.inner_html().to_string(), "\n    ", "");
        let description = str::replace(&desc, "&gt;", ">");
        let repo_description = str::replace(&description, "&lt;", "<");
        if !repo_description.contains("emoji") {
            println!("{}", repo_description);
        }
    }
}

