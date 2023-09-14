use crate::view::View;
mod view;
mod ui;

fn main() {
	let mut view = View::new();
	view.render().expect("Failed to run");
    println!("Hello, world!");
}
