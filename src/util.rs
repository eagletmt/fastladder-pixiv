extern crate chrono;
extern crate regex;
extern crate select;
extern crate serde_json;
extern crate url;

use chrono::TimeZone;

pub fn extract_pubdate(thumb: &str) -> chrono::DateTime<chrono::Local> {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"/img/(?P<year>\d{4})/(?P<month>\d{2})/(?P<day>\d{2})/(?P<hour>\d{2})/(?P<minute>\d{2})/(?P<second>\d{2})/").unwrap();
    }
    match RE.captures(thumb) {
        Some(caps) => {
            return chrono::Local
                .ymd(
                    caps["year"].parse().unwrap(),
                    caps["month"].parse().unwrap(),
                    caps["day"].parse().unwrap(),
                )
                .and_hms(
                    caps["hour"].parse().unwrap(),
                    caps["minute"].parse().unwrap(),
                    caps["second"].parse().unwrap(),
                );
        }
        None => {
            warn!("Unable to extract pubdate from {}", thumb);
            return chrono::Local.timestamp(0, 0);
        }
    }
}

#[derive(Debug, Deserialize)]
struct IllustItem {
    #[serde(rename = "illustId")]
    illust_id: String,
    #[serde(rename = "illustTitle")]
    illust_title: String,
    #[serde(rename = "userName")]
    user_name: String,
    url: String,
}
impl IllustItem {
    fn illust_url(&self) -> String {
        format!(
            "https://www.pixiv.net/member_illust.php?mode=medium&illust_id={}",
            self.illust_id
        )
    }
}

pub fn from_image_item(url: &url::Url, feedtitle: &str, doc: &select::document::Document) -> Vec<super::Feed> {
    from_data_items(url, feedtitle, doc, "js-mount-point-latest-following")
}

pub fn from_search_result(url: &url::Url, feedtitle: &str, doc: &select::document::Document) -> Vec<super::Feed> {
    from_data_items(url, feedtitle, doc, "js-mount-point-search-result-list")
}

fn from_data_items(url: &url::Url, feedtitle: &str, doc: &select::document::Document, id: &str) -> Vec<super::Feed> {
    let mut feeds = Vec::new();
    let div = doc.find(select::predicate::Attr("id", id)).next().expect(
        &format!(
            "Cannot find element with id={}",
            id,
        ),
    );
    let items = div.attr("data-items").expect(&format!(
        "Cannot find data-items attribute in {}",
        id,
    ));
    let result: Result<Vec<IllustItem>, _> = serde_json::from_str(&items);
    match result {
        Ok(items) => {
            for item in items {
                let pubdate = extract_pubdate(&item.url);
                let link = item.illust_url();
                let feed = super::Feed {
                    feedlink: url.to_string(),
                    feedtitle: feedtitle.to_owned(),
                    author: item.user_name,
                    title: item.illust_title,
                    thumb_url: url::Url::parse(&item.url).expect("data-items.url is unparsable"),
                    link: link,
                    category: "PxFeed".to_owned(),
                    published_date: pubdate.to_string(),
                };
                feeds.push(feed);
            }
        }
        Err(e) => {
            panic!("Cannot parse data-items: {}", e);
        }
    };
    feeds
}
