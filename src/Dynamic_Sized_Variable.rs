
fn main() {

    let name:&str="Shagun";
    println!("name is {:?}",name);

    let dynamic_name = String::from("Shagun Sethi");

    println!("dynamic_name is {:?}",dynamic_name);
    println!("New_dynamic_name stored in memory {:p}",&dynamic_name);

    let str_slice = &dynamic_name[0..6];
    println!("str_slice is {:?}",str_slice);

    let mut char= Vec::new();

    char.insert(0,'h');
    char.insert(1,'e');
    char.insert(2,'l');
    char.insert(3,'l');
    char.push('o');
    println!("chars is {:?}",char);
    dbg!(&char);

    let remmove_char =  char.pop().unwrap();
    println!("remmove_char {:?}", remmove_char);
    println!("char {:?}",char);







}  