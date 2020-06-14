mod guess;
use std::io;
use std::cmp::Ordering;

fn main() {
	let answer = guess::a_number();
	let mut step = 0;

	loop {
		step += 1;
		println!("Guess a number: ");

		let mut input = String::new();

		io::stdin().read_line(&mut input).expect("failed to input");

		let number = match input.trim().parse::<u16>(){
			Ok(num) => num,
			Err(_) => continue,
		};

		match answer.cmp(&number){
			Ordering::Less => print!("Smaller! "),
			Ordering::Greater => print!("Bigger! "),
			Ordering::Equal => {
				println!("Bingo! at step {}", step);
				break;
			},
		}
	}
}
