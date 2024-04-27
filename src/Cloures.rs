

fn main() {

    //Closures(Parameter of fucntion || shorthand function)
    let num =10;
    let add_num = |x:i32| x+num;

    let new_num = add_num(7);

    dbg!(new_num);


}  