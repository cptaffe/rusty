// stuff with rust

fn main() {

	let mut x = 3i;

	x = if x == 2 {
		5
	} else {
		println!("Hello");
		4
	};

	println!("x is {}", x)
}
