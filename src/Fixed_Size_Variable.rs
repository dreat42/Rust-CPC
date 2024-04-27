const TITLE: &str = "This is Sample Title";
 
fn main() {
    println!("The name of title one is {}!", TITLE);

    //stack
    let x:i32;
    x=2;
    println!("x is {}",x);

    let y:i32=4;
    println!("y is {}",y);

    //For loop

    for i in 0..=y{
        if i != 4{
            print!("{}  ",i)
        }
        else{
            print!("{} " ,i);
        }
    }

    //Mutation
    let mut z:i32=5;
    println!(" \n value of z is {}",z);
    z=10;
    println!("but is now {}",z);

    let Point_value = -12.3;
    println!("Value in Point value variable is {}",Point_value);

    let char_val = 'a';

    print!("charcter value is {}",char_val);

    let emoji_char = '😀';

    print!("Emoji Char is {}", emoji_char);

    let my_floats:[f32;10]= [0.3;10];

    print!("\n my_floats is {:?}",my_floats);

    let my_float_new:[f32;10] = my_floats.map(|n| n+0.2);

    print!("\n Value of my_float_new is {:?}",my_float_new);




}  