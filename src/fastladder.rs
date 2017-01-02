extern crate hyper;
extern crate rustc_serialize;
extern crate url;

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
        let client = hyper::Client::new();
        let url = self.base_url.join("/rpc/update_feeds").unwrap();
        let request_body = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("api_key", &self.api_key)
            .append_pair("feeds",
                         &rustc_serialize::json::encode(feeds).expect("Unable to encode feeds into JSON"))
            .finish();
        let _ = client.post(url).body(&request_body).send().expect("Failed to get");
    }
}
