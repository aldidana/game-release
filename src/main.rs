extern crate select;
extern crate reqwest;
#[macro_use] extern crate prettytable;

use std::error::Error;
use select::document::Document;
use select::predicate::{Predicate, Class, Name};
use std::vec::Vec;
use prettytable::Table;

fn main() {
    let request = get_release();
    match request {
        Ok(v) => println!("{}",v),
        Err(e) => println!("{}", e),
    }
}

fn get_release() ->  Result<String, Box<Error>>  {
    let res = reqwest::get("http://www.metacritic.com/browse/games/release-date/new-releases/all/date").unwrap();
    let document = Document::from_read(res).unwrap();

    let mut table = Table::new();

    for node in document.find(Class("list_product_condensed_module")) {
        let platform = node.find(Class("module_title").descendant(Name("a"))).next().unwrap();
        table.add_row(row![FwBdbc->platform.text(), FwBdbc->"Release Date"]);

        for games in node.find(Class("list_product_condensed")) {
            for game in games.find(Class("game_product")) {
                let title = game.find(Class("product_title").descendant(Name("a"))).next().unwrap();
                let game_title = title.text();
                let title_text = game_title.split_whitespace().collect::<Vec<_>>();
                let release_date = game.find(Class("data")).next().unwrap();
                
                table.add_row(row![title_text.join(" "), release_date.text()]);
            }
        }
    }
    table.printstd();
    Ok("Done".into())
  }