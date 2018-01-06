extern crate reqwest;
extern crate serde_json;
extern crate std;
extern crate url;

use std::io::Read;

pub struct Fastladder {
    base_url: url::Url,
    api_key: String,
}

impl Fastladder {
    pub fn new(base_url: url::Url, api_key: String) -> Fastladder {
        return Fastladder {
            base_url: base_url,
            api_key: api_key,
        };
    }

    pub fn post_feeds(&self, feeds: &Vec<super::Feed>) {
        let client = reqwest::Client::new();
        let url = self.base_url.join("/rpc/update_feeds").unwrap();
        let json_feeds = serde_json::to_string(feeds).expect("Unable to encode feeds into JSON");
        let mut params = std::collections::HashMap::new();
        params.insert("api_key", &self.api_key);
        params.insert("feeds", &json_feeds);
        let mut res = client.post(url).form(&params).send().expect(
            "Failed to get",
        );
        let mut response_body = String::new();
        let _ = res.read_to_string(&mut response_body).expect(
            "Failed to read body",
        );
        if !res.status().is_success() {
            // TODO: return Result
            panic!(format!(
                "fastladder/rpc/update_feeds returned {}: {}",
                res.status(),
                response_body
            ));
        }
    }
}
