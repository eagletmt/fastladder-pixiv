extern crate reqwest;
extern crate select;
extern crate url;

use std::io::Read;

pub fn bookmark_new_illust(base_url: &url::Url, phpsessid: &str) -> Result<Vec<super::Feed>, String> {
    let url = base_url.join("/bookmark_new_illust.php").unwrap();
    let feedtitle = "PxFeed - bookmark new illust";
    let client = reqwest::ClientBuilder::new()
        .redirect(reqwest::RedirectPolicy::none())
        .build()
        .expect("Failed to build reqwest::Client");
    let mut cookie = reqwest::header::Cookie::new();
    cookie.set("PHPSESSID".to_owned(), phpsessid.to_owned());
    let mut res = client.get(url.clone()).header(cookie).send().expect(
        "Failed to get",
    );
    let mut body = String::new();
    let _ = res.read_to_string(&mut body).expect("Failed to read body");
    if res.status().is_success() {
        let doc = select::document::Document::from(&*body);
        return Ok(super::util::from_image_item(&url, feedtitle, &doc));
    } else {
        // TODO: return Result
        return Err(format!(
            "/bookmark_new_illust.php returned {}: {}",
            res.status(),
            body
        ));
    }
}
