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
    fn serialize<S: serde::Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = try!(serializer.serialize_struct("Feed", 8));
        try!(serializer.serialize_struct_elt(&mut state, "feedlink", &self.feedlink));
        try!(serializer.serialize_struct_elt(&mut state, "feedtitle", &self.feedtitle));
        try!(serializer.serialize_struct_elt(&mut state, "author", &self.author));
        try!(serializer.serialize_struct_elt(&mut state, "title", &self.title));
        try!(serializer.serialize_struct_elt(&mut state,
                                             "body",
                                             format!("<img src=\"{}\"/>", self.thumb_url)));
        try!(serializer.serialize_struct_elt(&mut state, "link", &self.link));
        try!(serializer.serialize_struct_elt(&mut state, "category", &self.category));
        try!(serializer.serialize_struct_elt(&mut state, "published_date", &self.published_date));
        try!(serializer.serialize_struct_end(state));
        return Ok(());
    }
}
