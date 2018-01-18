struct A{}

impl A {
    fn a(&self){
        println!("Hello, A");
    }
}

impl A {
    fn b(&self){
        println!("Hello, B");
    }
}

trait Trait{
    fn mine(&self);
    fn mine2();
}

trait Trait2{
    fn nine();
    fn nine2();
    fn mine(&self);
}

impl Trait for A {
    fn mine(&self){
        println!("Trait 1");
    }
    fn mine2(){}
}

impl Trait2 for A {
    fn nine(){}
    fn nine2(){}
    fn mine(&self){
        println!("Trait 2");
    }
}

fn main(){
    let a = A{};
    
    a.a();
    a.b();

    //a.mine();
    Trait::mine(&a);
    Trait2::mine(&a);
}
