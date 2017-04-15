extern crate hyper;
extern crate hyper_rustls;
extern crate serde_json;
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
        let tls = hyper_rustls::TlsClient::new();
        let client = hyper::Client::with_connector(hyper::net::HttpsConnector::new(tls));
        let url = self.base_url.join("/rpc/update_feeds").unwrap();
        let request_body = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("api_key", &self.api_key)
            .append_pair("feeds",
                         &serde_json::to_string(feeds).expect("Unable to encode feeds into JSON"))
            .finish();
        let mut res = client
            .post(url)
            .body(&request_body)
            .send()
            .expect("Failed to get");
        let mut response_body = String::new();
        let _ = res.read_to_string(&mut response_body)
            .expect("Failed to read body");
        if res.status != hyper::status::StatusCode::Ok {
            // TODO: return Result
            panic!(format!("fastladder/rpc/update_feeds returned {}: {}",
                           res.status,
                           response_body));
        }
    }
}
