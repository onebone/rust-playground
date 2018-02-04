fn main(){
    let a = "Hello World";
    func(a);

    println!("{}", a);

    let b = "Hello 2";
    let b2 = b;
    println!("{}", b);

    let mut vec = Vec::new();
    if true {
    vec.push(1);
    }else{
        //vec.push("h");
    }
    //vec.push("hello");

    println!("{:?}", vec);

    let mut a = vec![0, 1, 2];
    let ref_a1 = &a;
    let ref_a2 = &a;
    // let ref_a2 = &mut a;
    // Mutable reference and immutable reference cannot coexist

    println!("{:?} {:?}", ref_a1, ref_a2);
}

fn func(a: &str){
    println!("{}", a);
}
