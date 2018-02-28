fn main(){
	println!("{}", a(match true {
		true => 6,
		false => 3
	}));
}

fn a(i: i32) -> bool {
	i > 5
}
