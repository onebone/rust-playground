fn main(){
    struct Point { x: i32, y: i32 }

    let p = Point{x: 2, y: 3};

    match p {
        Point{x, ..} => println!("{}", x)
    }

    let a = 1;
    match a {
        e@1...5 => println!("1 <= {} <= 5", e),
        _ => println!("else"),
    }
}
