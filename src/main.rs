use crate::view::View;

mod view;


fn main() {
	let view = View::new();
	view.run();
    println!("Hello, world!");
}
