use std::iter::Iterator;
use std::io;

fn main() {
    	let secret_word = vec!["s","a","l","u","t"];
	let mut view = Vec::new();
	for c in secret_word.iter() {
		view.push("_");
	}

	println!("{:?}",view);
	
	loop{

		println!("please choice a letter");

		let mut input = String::new();

		io::stdin()
		.read_line(&mut input)
		.expect("failed to read line");
		
		let guess = input.chars().next().unwrap().to_string();
		let guess_str: &str=&guess[..];
	
	 	for c in secret_word.iter() {
			if c == &guess_str {
			println!("{}",c);
			let index = secret_word.iter().position(|&x| &x == c).unwrap();
			let old_view = std::mem::replace(&mut view[index], c);
			println!("{}",index);	
			}
		}
		println!("{:?}",&view);
	}
}
