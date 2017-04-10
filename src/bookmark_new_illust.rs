extern crate cookie;
extern crate hyper;
extern crate select;
extern crate url;

use std::io::Read;

pub fn bookmark_new_illust(base_url: &url::Url, phpsessid: &str) -> Result<Vec<super::Feed>, String> {
    let url = base_url.join("/bookmark_new_illust.php").unwrap();
    let feedtitle = "PxFeed - bookmark new illust";
    let mut client = hyper::Client::new();
    client.set_redirect_policy(hyper::client::RedirectPolicy::FollowNone);
    let client = client;
    let mut res = client.get(url.clone()).header(hyper::header::Cookie(vec![cookie::Cookie::new("PHPSESSID".to_owned(), phpsessid.to_owned())])).send().expect("Failed to get");
    let mut body = String::new();
    let _ = res.read_to_string(&mut body)
        .expect("Failed to read body");
    if res.status == hyper::status::StatusCode::Ok {
        let doc = select::document::Document::from(&*body);
        return Ok(super::util::from_image_item(&url, feedtitle, &doc));
    } else {
        // TODO: return Result
        return Err(format!("/bookmark_new_illust.php returned {}: {}", res.status, body));
    }
}
