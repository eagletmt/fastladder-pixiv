#[derive(RustcEncodable)]
pub struct Feed {
    pub feedlink: String,
    pub feedtitle: String,
    pub author: String,
    pub title: String,
    pub body: String,
    pub link: String,
    pub category: String,
    pub published_date: String,
}
