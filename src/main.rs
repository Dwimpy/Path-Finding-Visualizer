use crate::view::View;
use crate::model::cell;
mod view;
mod model;

fn main() {
	let view = View::new();
	view.run();
    println!("Hello, world!");
}
