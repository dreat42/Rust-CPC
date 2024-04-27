#[cfg(test)]
mod test {
     use super::*;
    #[test]
    fn tests_pattern() {
         let number:i32 = 20;

        //  match number {
        //     1 => println!("TIt was the first!"),
        //     2 | 3 | 5 | 7 | 15 |20 => println!("We found it in the sequeunce !"),
        //     _=>println!("It was something else!"),
        //  }
       
        match number {
            1 => "TIt was the first!",
            2 | 3 | 5 | 7 | 15 |20 => "We found it in the sequeunce !",
            _=>"It was something else!",
         };
  

    }
} 