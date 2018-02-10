fn main(){
	let x = 5;
	let y = if x == 5 { 2.0 }else{ 1.0 };
	println!("Hello World, {}", y);
	/*let z = if false { 3 };
	println!("{}", z);*/ // ILLEGAL

	for x in 1..0 {
		println!("{}", x);
	}

    loop {
        break;
    }
}
