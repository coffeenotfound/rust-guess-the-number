use std::io;
use std::cmp::Ordering;
use rand::Rng;

const SECRET_NUMBER_RANGE: u32 = 100;

fn main() {
	// Generate the secret number
	let secret_number: u32 = rand::thread_rng().gen_range(1, SECRET_NUMBER_RANGE + 1);
	
	let mut first_try: bool = true;
	let mut num_tries: i32 = 0;
	
	loop {
		// Print text
		println!("{}", if first_try { "Try to guess my number:" } else { "Try again:" });
		
		// Read guess
		let mut raw_guess = String::new();
		
		io::stdin().read_line(&mut raw_guess)
			.expect("Failed to read line");
		
		// Parse guess
		let guess: u32 = raw_guess.trim().parse()
			.expect("Type in a number!");
		
		// Increment guess counter (no proper for-loop syntax, grrr)
		num_tries += 1;
		
		// Check number
		let compare_result = guess.cmp(&secret_number);
		
		// Print output
		let sentence = match compare_result {
			Ordering::Equal => "which is correct!",
			Ordering::Less => "but that's too small.",
			Ordering::Greater => "but that's too big.",
		};
		println!("You guessed {}, {}", guess, sentence);
		
		// Set first try flag
		first_try = false;
		
		// Break loop if guess was correct
		if compare_result == Ordering::Equal {
			break;
		}
	}
	
	// Print final text
	let final_sentence = if num_tries < 5 { ", good job!" } else { "." };
	println!("You took {} tries{}", num_tries, final_sentence);
}
