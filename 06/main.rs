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
}

fn func(a: &str){
    println!("{}", a);
}
