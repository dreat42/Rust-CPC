//Structs

#[derive(Debug)]
struct User{
username:String,
email:String,
sign_in_count:u64,
active:bool    
}

impl User{
    fn increment_sign_in_count(&mut self){
        self.sign_in_count +=1
    }

    fn change_email(&mut self , new_email:&str){
        self.email = String::from(new_email)
    }

    fn change_username(&mut self, new_username:&str){
        self.username = String::from(new_username);
    }
}

fn change_username(user:&mut User,new_username:&str){
    user.username = String::from(new_username)
}

#[cfg(test)]
mod test {
     use super::*;

    #[test]
    fn tests_structs() {
        let mut user_1 =User{
            username:String::from("shagunsethi"),
            email:String::from("shagunsethi@gmail.com"),
            active:true,
            sign_in_count:1
        };

       // user_1.username ="testusername".to_string();
       
       let user_1_ref =&mut user_1;
       change_username(user_1_ref,"newusername");
        dbg!(user_1);

         let mut user_2 =User{
            username:String::from("shagunsethi"),
            email:String::from("shagunsethi@gmail.com"),
            active:true,
            sign_in_count:1
        };

        user_2.increment_sign_in_count();
        user_2.change_email("shagunsethi3342@gmail.com");
          

        dbg!(user_2);
    }
}
