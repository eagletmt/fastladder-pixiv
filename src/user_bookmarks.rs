extern crate cookie;
extern crate hyper;
extern crate select;
extern crate url;

use select::predicate::Predicate;
use std::io::Read;

pub fn user_bookmarks(base_url: &url::Url, phpsessid: &str, user_id: &str) -> Vec<super::Feed> {
    let mut url = base_url.join("/bookmark.php").unwrap();
    url.query_pairs_mut().append_pair("id", user_id);
    let feedtitle = format!("PxFeed - Bookmarks by {}", user_id);
    let client = hyper::Client::new();
    let mut res = client.get(url.clone()).header(hyper::header::Cookie(vec![cookie::Cookie::new("PHPSESSID".to_owned(), phpsessid.to_owned())])).send().expect("Failed to get");
    let mut body = String::new();
    let _ = res.read_to_string(&mut body).expect("Failed to read body");
    let doc = select::document::Document::from(&*body);
    let mut feeds = Vec::new();
    for li in doc.find(select::predicate::Class("image-item")).iter() {
        let title_node = li.find(select::predicate::Class("title")).first().expect("Unable to find title node");
        let title = title_node.text();
        let thumb_node = li.find(select::predicate::Name("img")).first().expect("Unable to find thumbnail node");
        let thumb = thumb_node.attr("data-src").expect("data-src does not exist in thumbnail node");
        let user_node = li.find(select::predicate::Class("user")).first().expect("Unable to find user node");
        let user = user_node.attr("data-user_name").expect("data-user_name does not exist in user node");
        let link_node = li.find(select::predicate::Class("work").and(select::predicate::Name("a"))).first().expect("Unable to find a.work node");
        let link = url.join(link_node.attr("href").expect("href does not exist in a.work node")).expect("Unable to join href in a.work node");
        feeds.push(super::Feed {
            feedlink: url.to_string(),
            feedtitle: feedtitle.to_string(),
            author: user.to_string(),
            title: title,
            body: format!("<img src=\"{}\"/>", thumb),
            link: link.to_string(),
            category: "PxFeed".to_string(),
            published_date: super::util::extract_pubdate(thumb).to_string(),
        });
    }
    return feeds;
}
