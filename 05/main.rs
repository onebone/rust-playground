struct Point {
    x: i32,
    y: i32
}

impl<'a> Point {
    fn a(&self) -> &'a str {
        "Hello World 2"
    }
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
    
    let mut s = "";
    {
        let p2 = Point{x: 5, y: 6};
        s = p2.a();
    }

    println!("{}", s);

    let mut i = 5;
    {
        let y = &mut i;
        *y += 1;
        //y += 1;
    }
    println!("{}", i);
}

