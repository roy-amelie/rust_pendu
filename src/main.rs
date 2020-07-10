use std::iter::Iterator;
use std::io;

fn main() {
    	let secret_word = "bonjour";
	let bytes=secret_word.as_bytes();
	let mut view = Vec::new();
	for c in bytes.iter().enumerate() {
		view.push('_');
	}

	println!("{:?}",view);
	
	loop{

		println!("please choice a letter");

		let mut input = String::new();

		io::stdin()
		.read_line(&mut input)
		.expect("failed to read line");
		
		let guess = input.chars().next().unwrap();
	
	 	for (i,&c) in bytes.iter().enumerate() {
			if c as char == guess {
			let old_view = std::mem::replace(&mut view[i], c as char);
			println!("{}",i);	
			}
		}
		println!("{:?}",&view);
		let mut count =0;
		for item in &view {
			if item == &'_' {
				count +=1;
			}
		}
		if count == 0 {
			break;
		}
	}
}
