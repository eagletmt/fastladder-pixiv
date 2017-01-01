extern crate chrono;
extern crate regex;

use chrono::TimeZone;

pub fn extract_pubdate(thumb: &str) -> chrono::DateTime<chrono::Local> {
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
