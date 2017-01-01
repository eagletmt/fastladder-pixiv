extern crate hyper;
extern crate select;
extern crate std;
extern crate url;

use select::predicate::Predicate;
use std::io::Read;

pub fn search_by_tag(base_url: &url::Url, word: &str) -> Vec<super::Feed> {
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
        let title_node = li.find(select::predicate::Class("title")).first().expect("Unable to find title node");
        let title = title_node.attr("title").expect("title attribute does not exist in title node");
        let thumb_node = li.find(select::predicate::Class("_thumbnail"))
            .first()
            .expect("Unable to find thumbnail node");
        let thumb = thumb_node.attr("data-src")
            .expect("data-src attribute does not exist in thumbnail node");
        let user_node = li.find(select::predicate::Class("user")).first().expect("Unable to find user node");
        let user = user_node.attr("title").expect("title attribute does not exist in user node");
        let link_node = li.find(select::predicate::Class("work").and(select::predicate::Name("a")))
            .first()
            .expect("Unable to find a.work node");
        let link = base_url.join(link_node.attr("href").expect("href does not exist in a.work node"))
            .expect("Unable to join href in a.work node");
        let pubdate = super::util::extract_pubdate(thumb);
        let feed = super::Feed {
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
