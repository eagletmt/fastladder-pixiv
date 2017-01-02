extern crate env_logger;
extern crate fastladder_pixiv;
extern crate rustc_serialize;
extern crate url;
#[macro_use]
extern crate clap;

use std::io::Write;

fn main() {
    env_logger::init().unwrap();

    let app = clap_app!(fastladder_pixiv =>
        (version: "0.1.0")
        (about: "Post pixiv feeds to fastladder")
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

    match run_subcommand(&base_url, &app, matches.subcommand()) {
        Ok(feeds) => {
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
        Err(msg) => {
            let _ = writeln!(&mut std::io::stderr(), "{}", msg);
            std::process::exit(1);
        }
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

fn run_subcommand(base_url: &url::Url, app: &clap::App, subcommand: (&str, Option<&clap::ArgMatches>)) -> Result<Vec<fastladder_pixiv::Feed>, String> {
    let mut feeds = Vec::new();

    match subcommand {
        ("word", Some(word_command)) => {
            for word in word_command.values_of("WORD").unwrap() {
                feeds.append(&mut try!(fastladder_pixiv::search_by_tag(&base_url, word)));
            }
        }
        ("bookmark", Some(_)) => {
            let phpsessid = std::env::var("PIXIV_PHPSESSID").expect("PHPSESSID is required for bookmark subcommand");
            feeds.append(&mut try!(fastladder_pixiv::bookmark_new_illust(&base_url, &phpsessid)));
        }
        ("user", Some(user_command)) => {
            let phpsessid = std::env::var("PIXIV_PHPSESSID").expect("PHPSESSID is required for user subcommand");
            for user_id in user_command.values_of("USER_ID").unwrap() {
                feeds.append(&mut try!(fastladder_pixiv::user_bookmarks(&base_url, &phpsessid, user_id)));
            }
        }
        _ => {
            let _ = app.write_help(&mut std::io::stderr());
            let _ = std::io::stderr().write(b"\n");
            std::process::exit(1);
        }
    };

    return Ok(feeds);
}
