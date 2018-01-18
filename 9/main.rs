fn main(){
    let mut a = 5;

    {
        let mut add_num = move |x| a += x;

        add_num(5);
    }

    println!("{}", a);



    let mut vec = vec![1, 2, 3];

    {
        //let mut push = move |x| vec.push(x);
        let mut push = |x| vec.push(x);
        push(4);
    }

    println!("{:?}", vec);
}
