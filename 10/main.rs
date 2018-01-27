macro_rules! foo {
    ($x:expr) => (println!("Hello World X {}", $x));
//    (y => $y:expr) => (println!("Hello World Y {}", $y));
}

macro_rules! bar {
    ( $( $x:expr ),* ) => {
        $(
            println!("{}", $x);
        )*
    };
}

macro_rules! foobar {
    ( $( {$x:expr} ),*) => {
        $(
            println!("{}", $x);
        )*
    };
}

macro_rules! wow {
    ( $( $x:expr ) * ) => {
        $(
            println!("{}", $x);
        )*
    };
}

fn main(){
    foo!(1); 
    bar!(1, 2, 3);
    foobar!({4}, {5}, {6});
    wow!(7 8 9);
}
