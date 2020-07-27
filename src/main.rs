#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::Redirect;
mod utils;
use utils::command::Command;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    println!("You typed in :{}", cmd);

    let command = utils::get_command_from_query_string(&cmd);

    let redirect_url = match command {
        command => match Some(Command{cmd, rest}) {
            cmd, rest if cmd == "add" => todo!(),
            Command{cmd, rest} => utils::generic::construct_generic_url(cmd),
        }
        None => utils::google::construct_google_search_url(&cmd)
    };
    Redirect::to(redirect_url)
}

fn main(){
    rocket::ignite().mount("/", routes![index, search]).launch();
}
