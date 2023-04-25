use hello_macro::HelloMacro;

use hello_macro_derive::route;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[route(GET, "/")]
fn index() {}

fn main() {
    Pancakes::hello_macro();
    index();
}
