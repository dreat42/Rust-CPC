
#[allow(dead_code,unused_varibles)]
fn example_0(){
    let r:&i32; //'a

   // {
        let x:i32=5; //   'b
        r= &x;
   // }
    println!("r: {}",r);
}


#[allow(dead_code,unused_varibles)]
fn example_1(){
    //Allocate space in memory
    let highest_age:i32;

    //Intialize vars
    let alice_age:i32=20;
    let bob_age:i32=21;

    //Call functions
    highest_age = largest(&alice_age,&bob_age);

    //Print output
    println!("Highest_age is {}",highest_age);

    fn largest(compare_1:&i32,compare_2:&i32) -> i32{
        if compare_1 > compare_2{
            *compare_1
        }
        else{
            *compare_2
        }
    }

}




#[allow(dead_code,unused_varibles)]
fn example_3(){
    //Allocate space in memory
    let highest_age:&i32;

    //Intialize vars
    let alice_age:i32=20; //'a
    let bob_age:i32=21; // 'b:'a

    //Call functions
    highest_age = largest(&alice_age,&bob_age);

    //Print output
    println!("Highest_age is {}",highest_age);

    fn largest<'a,'b:'a>(compare_1:&'a i32,compare_2:&'b i32) -> &'a i32{
        if compare_1 > compare_2{
            compare_1
        }
        else{
            compare_2
        }
    }

}



#[allow(dead_code,unused_varibles)]
fn example_4(){
    //Allocate space in memory
    let highest_age:&i32;

    let new_val:i32;

    //Intialize vars
    let alice_age:i32=20; //'a
    {
    let bob_age:i32=21; // 'b
     //Call functions
    highest_age = largest(&alice_age,&bob_age);
     
     new_val = *highest_age;
     // 'b out of scope
    } 

   
    //Print output
    println!("Highest_age is {}",  new_val);

    fn largest<'a>(compare_1:&'a i32,compare_2:&'a i32) -> &'a i32{
        if compare_1 > compare_2{
            compare_1
        }
        else{
            compare_2
        }
    }

}


#[allow(dead_code,unused_varibles)]
fn example_5_generics(){
    //Allocate space in memory
    let highest_age:&i32;

    let new_val:i32;

    //Intialize vars
    let alice_age:i32=20; //'a
    {
    let bob_age:i32=21; // 'b
     //Call functions
    highest_age = largest::<i32>(&alice_age,&bob_age);
     
     new_val = *highest_age;
     // 'b out of scope
    } 

   
    //Print output
    println!("Highest_age is {}",  new_val);

    fn largest<'a,T:PartialOrd>(compare_1:&'a T,compare_2:&'a T) -> &'a T{
        if compare_1 > compare_2{
            compare_1
        }
        else{
            compare_2
        }
    }

}

#[allow(dead_code,unused_varibles)]
struct Person<'p>{
    name:&'p str,
    points:&'p f32
}

#[allow(dead_code,unused_varibles)]
fn example_6_with_struct(){
    //Allocate space in memory
    let highest_age:&f32;

    let new_val:f32;

    //Intialize vars
    let alice:Person=Person{name:"alice",points:&50.2}; 
    {
    let bob_age:i32=21; // 'b

      let bob:Person=Person{name:"bob",points:&40.5}; 
     //Call functions
    highest_age = largest::<f32>(alice.points,bob.points);
     
     new_val = *highest_age;
     // 'b out of scope
    } 

   
    //Print output
    println!("Highest_age is {}",  new_val);

    fn largest<'a,T:PartialOrd>(compare_1:&'a T,compare_2:&'a T) -> &'a T{
        if compare_1 > compare_2{
            compare_1
        }
        else{
            compare_2
        }
    }

}
