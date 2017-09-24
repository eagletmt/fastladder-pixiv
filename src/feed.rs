extern crate serde;
extern crate url;

#[derive(Debug)]
pub struct Feed {
    pub feedlink: String,
    pub feedtitle: String,
    pub author: String,
    pub title: String,
    pub thumb_url: url::Url,
    pub link: String,
    pub category: String,
    pub published_date: String,
}

impl serde::Serialize for Feed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use feed::serde::ser::SerializeStruct;

        let imgurl = self.thumb_url
            .as_str()
            .replace("150x150", "600x600")
            .replace("240x240", "600x600");
        let mut struc = try!(serializer.serialize_struct("Feed", 8));
        try!(struc.serialize_field("feedlink", &self.feedlink));
        try!(struc.serialize_field("feedtitle", &self.feedtitle));
        try!(struc.serialize_field("author", &self.author));
        try!(struc.serialize_field("title", &self.title));
        try!(struc.serialize_field(
            "body",
            &format!("<img src=\"{}\"/>", imgurl),
        ));
        try!(struc.serialize_field("link", &self.link));
        try!(struc.serialize_field("category", &self.category));
        try!(struc.serialize_field(
            "published_date",
            &self.published_date,
        ));
        struc.end()
    }
}
