extern crate hyper;
extern crate select;
extern crate std;
extern crate url;

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
    return super::util::from_image_item(&url, &feedtitle, &doc);
}
