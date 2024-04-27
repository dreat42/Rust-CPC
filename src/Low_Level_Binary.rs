

fn main() {

    let a:u8 =0b_1010_1010;
    let b:u8 =0b_0101_1010;


    println!("a's Value is {}",a);
    println!("b's Value is {}",b);

    println!("a's Value is {:08b}",a);
    println!("b's Value is {:08b}",b);

    //Logic Gates
    println!("And {:08b}",a & b);
    println!("Or {:08b}",a | b);
    println!("XOR {:08b}",a ^ b);
    println!("NOT {:08b}", !a );
             
    //BitWise Operations
    println!("a << 1 {:08b}", a << 1);
    println!("a << 1 {}",a << 1);
    println!("a >> 1 {:08b}", a >> 1);
    println!("a << 1 {}",a >>  1);

   //Little Endian or Big Indian
   let n: u16 = 0x1234;
    println!("n is {:?}", n);

    let big_endian = n.to_be_bytes();
    let little_endian = n.to_le_bytes();

    println!("n in big endian: {:02X}{:02X}", big_endian[0], big_endian[1]);
    println!("n in little endian: {:02X}{:02X}", little_endian[0], little_endian[1]);
}  