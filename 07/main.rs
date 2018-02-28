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

struct Var {
	a: i64,
}

trait Abstract {
	fn set_a(&mut self, a: i64) {
		self.a = a;
	}

	fn get_a(&self) -> i64 {
		self.a
	}
}

impl Abstract for Var {
}

fn main(){
    let a = A{};
    
    a.a();
    a.b();

    //a.mine();
    Trait::mine(&a);
    Trait2::mine(&a);

	let v = Var{a: 4};
	println!("{}", v.get_a());
}
