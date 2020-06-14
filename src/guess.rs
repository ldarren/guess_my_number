use rand::Rng;

pub fn a_number() -> u16 {
	let number = rand::thread_rng().gen_range(0, 101);
	number as u16
}
