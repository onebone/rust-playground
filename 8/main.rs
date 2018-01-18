fn main(){
    let mut v = vec![1, 2, 3, 4, 5];

    println!("{:?}", v.pop());

    if let Some(x) = v.pop() {
        println!("{}", x);
    }

    while let Some(x) = v.pop() {
        println!("{:?}", x);
    }
}
