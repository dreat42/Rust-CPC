

fn main() {


//Number Literals 
println!("Big Number is {}",98_222_000);
println!("Hex is {}",0xff);
println!("Octal is {}",0o77);
print!("Binary is {}",0b1111_0000);
println!("Bytes 'A' is {}",b'A');

//Raw - String Literal

let test:&str = r#"{\"message":"This is a test message"}"#;
dbg!(test);

}  