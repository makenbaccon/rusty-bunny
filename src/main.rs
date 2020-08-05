#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::Redirect;

mod utils;

#[get("/")]
fn index() -> &'static str {
    "TODO: add database and interface to add bookmarks"
}
// rename cmd to query
#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    // We need a way to match only on the cmd, without losing the rest of the query
    // "tw something"
    let command = utils::get_command_from_query_string(&cmd);

    // Keep in alphabetic order
    let redirect_url = match command {
        // google tools
        "cal" => String::from("https://calendar.google.com/"),
        "drive" => String::from("https://drive.google.com/"),
        "mail" => String::from("https://mail.google.com/"),
        "msg" => String::from("https://messages.google.com/web/conversations"),
        // coding tools
        "gh" => utils::github::construct_github_url(&cmd),
        "ih" => String::from("https://indiehackers.com"),
        "l8" => String::from("http://localhost:8000/"),
        "hk" => String::from("http://https://dashboard.heroku.com/"),
        // fun tools
        "yt" => String::from("https://www.youtube.com/feed/subscriptions"),
        "rdt" => String::from("https://www.reddit.com/"),
        // misc tools
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        // If no match, we search on Google
        _ => utils::google::construct_google_search_url(&cmd),
    };

    Redirect::to(redirect_url)
}
#[rustfmt::skip]
fn main() {
    rocket::ignite()
        .mount("/", routes![index, search])
        .launch();
}
