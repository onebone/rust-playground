fn main(){
	let x = 5;
	let y = if x == 5 { 2.0 }else{ 1.0 };
	println!("Hello World, {}", y);

	for x in 1..0 {
		println!("{}", x);
	}

    loop {
        break;
    }
}
