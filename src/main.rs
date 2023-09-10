use crate::view::View;
mod view;
mod model;
mod ui;

fn main() {
	let mut view = View::new();
	view.render().expect("Failed to run");
    println!("Hello, world!");
}
