use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Name;
use url::ParseError as UrlParseError;
use url::Url;

pub struct LinkExtractor {
    client: Client,
}

impl LinkExtractor {
    pub fn from_client(client: Client) -> Self {
        Self { client }
    }

    pub fn get_links(&self, url: Url) -> Result<Vec<Url>, eyre::Report> {
        log::info!("GET \"{}\"", url);
        let response = self.client.get(url).send()?;
        let base_url = response.url().clone();
        let status = response.status();
        let body = response.text()?;
        let doc = Document::from(body.as_str());

        let mut links = Vec::new();
        log::info!("Retrieved {} \"{}\"", status, base_url);
        for href in doc.find(Name("a")).filter_map(|a| a.attr("href")) {
            match Url::parse(href) {
                Ok(url) => {
                    links.push(url);
                }
                Err(UrlParseError::RelativeUrlWithoutBase) => {
                    // `href` を絶対URLに変換する
                    let url = base_url.join(href)?;
                    links.push(url);
                }
                Err(e) => {
                    println!("Error:{}", e);
                }
            };
        }
        Ok(links)
    }
}
