struct Point {
    x: i32,
    y: i32
}

fn get_str<'a>() -> &'a str {
    "Hello World"
}

fn main(){
    println!("{}", get_str());

    let mut p = Point { x: 2, y: 3 };
    {
        let mut p1 = &mut p;

        p1.x = 5;
    }

    //p = p1;
    println!("{} {}", p.x, p.y);
}

