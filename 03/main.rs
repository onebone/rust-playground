fn main(){
    struct Circle(i32, i32);

    let circle: Circle = Circle(1, 2);
    println!("{} {}", circle.0, circle.1);

    struct Electron{}
    struct Proton;

    let electron = Electron{};
    // let electron2 = Electron;

    let proton = Proton{};
    let proton2 = Proton;
}
