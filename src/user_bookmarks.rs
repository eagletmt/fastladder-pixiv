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
    let mut client = hyper::Client::new();
    client.set_redirect_policy(hyper::client::RedirectPolicy::FollowNone);
    let client = client;
    let mut res = client.get(url.clone()).header(hyper::header::Cookie(vec![cookie::Cookie::new("PHPSESSID".to_owned(), phpsessid.to_owned())])).send().expect("Failed to get");
    let mut body = String::new();
    let _ = res.read_to_string(&mut body).expect("Failed to read body");
    if res.status == hyper::status::StatusCode::Ok {
        let doc = select::document::Document::from(&*body);
        let mut feeds = Vec::new();
        for li in doc.find(select::predicate::Class("image-item")).iter() {
            let title_node = li.find(select::predicate::Class("title")).first().expect("Unable to find title node");
            let title = title_node.text();
            let thumb_node = li.find(select::predicate::Name("img")).first().expect("Unable to find thumbnail node");
            let thumb = thumb_node.attr("data-src").expect("data-src does not exist in thumbnail node");
            let thumb_url = url::Url::parse(thumb).expect("data-src in thumbnail node is unparsable");
            let user_node = li.find(select::predicate::Class("user")).first().expect("Unable to find user node");
            let user = user_node.attr("data-user_name").expect("data-user_name does not exist in user node");
            let link_node = li.find(select::predicate::Class("work").and(select::predicate::Name("a"))).first().expect("Unable to find a.work node");
            let link = url.join(link_node.attr("href").expect("href does not exist in a.work node")).expect("Unable to join href in a.work node");
            feeds.push(super::Feed {
                feedlink: url.to_string(),
                feedtitle: feedtitle.to_owned(),
                author: user.to_owned(),
                title: title,
                thumb_url: thumb_url,
                link: link.to_string(),
                category: "PxFeed".to_owned(),
                published_date: super::util::extract_pubdate(thumb).to_string(),
            });
        }
        return feeds;
    } else {
        // TODO: return Result
        panic!(format!("fastladder/rpc/update_feeds returned {}: {}",
                       res.status,
                       body));
    }
}
