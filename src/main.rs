extern crate chrono;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate select;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate url;

use std::io::Read;
use select::predicate::Predicate;
use chrono::TimeZone;
use std::io::Write;

fn main() {
    env_logger::init().unwrap();

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        let _ = writeln!(&mut std::io::stderr(), "Usage: {} WORD", args[0]);
        std::process::exit(1);
    }

    let base_url = url::Url::parse("http://www.pixiv.net").unwrap();
    let feeds = search_by_tag(&base_url, &args[1]);
    println!("{}",
             rustc_serialize::json::encode(&feeds).expect("Unable to encode feeds into JSON"));
}

#[derive(RustcEncodable)]
struct Feed {
    feedlink: String,
    feedtitle: String,
    author: String,
    title: String,
    body: String,
    link: String,
    category: String,
    published_date: String,
}

fn search_by_tag(base_url: &url::Url, word: &str) -> Vec<Feed> {
    let mut url = base_url.join("/search.php").unwrap();
    url.query_pairs_mut().append_pair("s_mode", "s_tag").append_pair("word", &word);
    let feedtitle = format!("PxFeed - {}", word);
    let client = hyper::Client::new();
    let mut res = client.get(url.clone()).send().expect("Failed to get");
    let mut body = String::new();
    let _ = res.read_to_string(&mut body).expect("Failed to read body");
    let doc = select::document::Document::from(&*body);
    let mut feeds = std::vec::Vec::new();
    for li in doc.find(select::predicate::Class("image-item")).iter() {
        let title_node =
            li.find(select::predicate::Class("title")).first().expect("Unable to find title node");
        let title = title_node.attr("title").expect("title attribute does not exist in title node");
        let thumb_node = li.find(select::predicate::Class("_thumbnail"))
            .first()
            .expect("Unable to find thumbnail node");
        let thumb = thumb_node.attr("data-src")
            .expect("data-src attribute does not exist in thumbnail node");
        let user_node =
            li.find(select::predicate::Class("user")).first().expect("Unable to find user node");
        let user = user_node.attr("title").expect("title attribute does not exist in user node");
        let link_node = li.find(select::predicate::Class("work").and(select::predicate::Name("a")))
            .first()
            .expect("Unable to find a.work node");
        let link =
            base_url.join(link_node.attr("href").expect("href does not exist in a.work node"))
                .expect("Unable to join href in a.work node");
        let pubdate = extract_pubdate(thumb);
        let feed = Feed {
            feedlink: url.to_string(),
            feedtitle: feedtitle.to_string(),
            author: user.to_string(),
            title: title.to_string(),
            body: format!("<img src=\"{}\"/>", thumb),
            link: link.to_string(),
            category: "PxFeed".to_string(),
            published_date: pubdate.to_string(),
        };
        feeds.push(feed)
    }
    return feeds;
}

fn extract_pubdate(thumb: &str) -> chrono::DateTime<chrono::Local> {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"/img/(?P<year>\d{4})/(?P<month>\d{2})/(?P<day>\d{2})/(?P<hour>\d{2})/(?P<minute>\d{2})/(?P<second>\d{2})/").unwrap();
    }
    match RE.captures(thumb) {
        Some(caps) => {
            return chrono::Local.ymd(caps["year"].parse().unwrap(),
                     caps["month"].parse().unwrap(),
                     caps["day"].parse().unwrap())
                .and_hms(caps["hour"].parse().unwrap(),
                         caps["minute"].parse().unwrap(),
                         caps["second"].parse().unwrap());
        }
        None => {
            warn!("Unable to extract pubdate from {}", thumb);
            return chrono::Local.timestamp(0, 0);
        }
    }
}
