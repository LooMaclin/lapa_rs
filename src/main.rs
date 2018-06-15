extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate rustbreak;

use rustbreak::Database;

#[derive(Serialize, Deserialize)]
struct Foo { x: u8 }

fn main() {
    let config = Database::open("test-db").unwrap();
    config.insert("foo", Foo { x: 42 });
}