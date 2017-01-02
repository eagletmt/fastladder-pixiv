extern crate rustc_serialize;
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

impl rustc_serialize::Encodable for Feed {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("Feed", 8, |s| {
            try!(s.emit_struct_field("feedlink", 0, |s| self.feedlink.encode(s)));
            try!(s.emit_struct_field("feedtitle", 1, |s| self.feedtitle.encode(s)));
            try!(s.emit_struct_field("author", 2, |s| self.author.encode(s)));
            try!(s.emit_struct_field("title", 3, |s| self.title.encode(s)));
            try!(s.emit_struct_field("body",
                                     4,
                                     |s| format!("<img src=\"{}\"/>", self.thumb_url).encode(s)));
            try!(s.emit_struct_field("link", 5, |s| self.link.encode(s)));
            try!(s.emit_struct_field("category", 6, |s| self.category.encode(s)));
            try!(s.emit_struct_field("published_date", 7, |s| self.published_date.encode(s)));
            Ok(())
        })
    }
}
