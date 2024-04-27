

fn main() {
    let char: Vec<&str> = vec!["h", "e", "l","l","o"];

    println!("{:?}", char);


   char.iter().for_each(|c| print!("{}",c));

    let char_again: Vec<char> = vec!['h', 'e', 'l','l','o'];

    println!("{:?}", &char_again);

    let collections:String = char_again.iter().collect();

    dbg!(collections);
     

     for c in char_again{
        print!("{}",c);
        if c=='o'{
            print!(", world")
        }
     }

}  