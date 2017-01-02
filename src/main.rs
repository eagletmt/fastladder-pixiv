extern crate env_logger;
extern crate fastladder_pixiv;
extern crate rustc_serialize;
extern crate url;
#[macro_use]
extern crate clap;

use std::io::Write;

fn main() {
    env_logger::init().unwrap();

    let app = clap_app!(myapp =>
        (@arg dry_run: -n "dry-run")
        (@subcommand word =>
            (about: "Search illustrations by tag")
            (@arg WORD: +required +multiple "Word"))
        (@subcommand bookmark =>
            (about: "Get new illustrations from following users"))
        (@subcommand user =>
            (about: "Get new illustrations from user's bookmark")
            (@arg USER_ID: +required +multiple "user id"))
    );
    let matches = app.clone().get_matches();
    let dry_run = matches.is_present("dry_run");
    let base_url = url::Url::parse("http://www.pixiv.net").unwrap();

    let feeds: Vec<fastladder_pixiv::Feed> = match matches.subcommand() {
        ("word", Some(word_command)) => {
            word_command.values_of("WORD")
                .unwrap()
                .flat_map(|word| fastladder_pixiv::search_by_tag(&base_url, word))
                .collect()
        }
        ("bookmark", Some(_)) => {
            fastladder_pixiv::bookmark_new_illust(&base_url,
                                                  &std::env::var("PIXIV_PHPSESSID").expect("PHPSESSID is required for bookmark subcommand"))
        }
        ("user", Some(user_command)) => {
            let phpsessid = std::env::var("PIXIV_PHPSESSID").expect("PHPSESSID is required for bookmark subcommand");
            user_command.values_of("USER_ID").unwrap().flat_map(|user_id| fastladder_pixiv::user_bookmarks(&base_url, &phpsessid, user_id)).collect()
        }
        _ => {
            let _ = app.write_help(&mut std::io::stderr());
            let _ = std::io::stderr().write(b"\n");
            std::process::exit(1);
        }
    };
    let feeds = feeds.into_iter().map(|feed| replace_host(feed)).collect();
    if dry_run {
        println!("{}",
                 rustc_serialize::json::encode(&feeds).expect("Unable to encode feeds into JSON"));
    } else {
        let api_key = std::env::var("FASTLADDER_API_KEY").expect("FASTLADDER_API_KEY is required to post feeds");
        let fastladder_url = std::env::var("FASTLADDER_URL").expect("FASTLADDER_URL is required to post feeds");
        let fastladder = fastladder_pixiv::Fastladder::new(url::Url::parse(&fastladder_url).expect("Unparsable FASTLADDER_URL"),
                                                           api_key);
        fastladder.post_feeds(&feeds);
    }
}

fn replace_host(feed: fastladder_pixiv::Feed) -> fastladder_pixiv::Feed {
    let mut replaced = feed;
    if let Ok(replace_url_str) = std::env::var("REPLACE_URL") {
        if let Ok(replace_url) = url::Url::parse(&replace_url_str) {
            replaced.thumb_url.set_host(replace_url.host_str()).expect("Unable to replace host");
            replaced.thumb_url.set_scheme(replace_url.scheme()).expect("Unable to replace scheme");
        }
    }
    return replaced;
}
