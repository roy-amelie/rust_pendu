use std::iter::Iterator;
use std::io;

fn main() {
    	let secret_letter = 'b';

	loop{

		println!("please choice a letter");

		let mut input = String::new();

		io::stdin()
		.read_line(&mut input)
		.expect("failed to read line");
		
		let guess_vec: Vec<char> = input.chars().collect();
		let guess = guess_vec[0];
	 	if secret_letter == guess {
			println!("bravo vous avez trouver la lettre {}",secret_letter);
			break;
		} else {
			println!("ce n'est pas la bonne lettre")
		}
	}
}
