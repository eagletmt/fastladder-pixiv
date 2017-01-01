extern crate env_logger;
extern crate fastladder_pixiv;
extern crate rustc_serialize;
extern crate url;
#[macro_use]
extern crate clap;

fn main() {
    env_logger::init().unwrap();

    let app = clap_app!(myapp =>
        (@subcommand word =>
            (about: "Search illustrations by tag")
            (@arg WORD: +required +multiple "Word")
        )
    );
    let matches = app.clone().get_matches();
    let base_url = url::Url::parse("http://www.pixiv.net").unwrap();

    let feeds: Vec<fastladder_pixiv::Feed> = match matches.subcommand() {
        ("word", Some(word_command)) => {
            word_command.values_of("WORD")
                .unwrap()
                .flat_map(|word| fastladder_pixiv::search_by_tag(&base_url, word))
                .collect()
        }
        _ => {
            let _ = app.write_help(&mut std::io::stderr());
            std::process::exit(1);
        }
    };
    println!("{}",
             rustc_serialize::json::encode(&feeds).expect("Unable to encode feeds into JSON"));
}
