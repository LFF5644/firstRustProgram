use rand::Rng;
use std::ops::RangeInclusive;

fn main(){
	println!("Random Number: {:2}",gen_number());
	loop{
		print!("{:02} ",gen_number());
	}
}

fn gen_number()->i32{
	let mut rng=rand::thread_rng();
	return rng.gen_range(RangeInclusive::new(1,100));
}
