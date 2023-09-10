use crate::view::View;
use crate::model::cell;
mod view;
mod model;

fn main() {
	let mut view = View::new();
	view.render().unwrap();
    println!("Hello, world!");
}
