// use std::io;

pub fn variables () {
    // scalar and compound type
    /* 

    *------------------------------*
    *  scalar      *  compound     *
    *------------------------------*
    *  String      *   Tuple       *
    *  Integer     *   Array       *
    *  Char        *   Vector      *
    *  Bool        *               *
    *--------------*---------------*

    */

    // scalar start here
    let mut a: u32 = 256;   // by default int u32 && without mut we can't change the value of a
    println!("first a value is {}", a); // we get warning if we not using before change the value
    a = 500;
    println!("second a value is {}", a);
    let f = 2.1090; // by default float f64
    println!("float value is {}", f);

    let a = "five";  // shadowing of a;

    println!("variables is {}", a);

    let b = true;
    // b = false;

    println!("The boolean value is {}", b);

    let c = '㋡';
    println!("The char is {}", c);

    // difference between CONST and normal variables are 
    // 1) We must specify the data type for const
    // 2) We can't use mut in const variable
    const COUNT: u32 = 10; 

    println!("count print is {}", COUNT);

    // key to note
    // String::new() is used for dynamic value (runtime value)
    // str is user for static value (compiletime value)

    // scalar end here

 // -----------------------------------------------------------------------------------------------------------------------

    // compound start here

    let tup = ("rust is powerful", 200); // this is tuple --> first and second value may be differ in type

    let (first_value, second_value) = tup; // tupe destructring value
    println!("Tuple print is {} and second value is {}", first_value, second_value );

    let first_val = tup.0; // get the first tup value
    let second_val = tup.1; // get the second tup value

    println!("First val is {} and second val is {}", first_val, second_val);

    
    let arr = [100,200,300,400,500,600];  // array declartion

    // arr.len(); // to findouut an length of an array

    println!("the length of array is {}", arr.len());
    // look like for each --> a.iter() is important to note
    for e in arr.iter() {
        println!("Element in array {}", e);
    }

    // for loop with number range
    for e in 1..6 {
        println!("Iterate throw number range {}", e);
    }

    // while loop
    let mut a = 5;
    while a!=0 {
        println!("{}",a);
        a -=1;
    }

    let data = 5;

    if data < 5 {
        println!("A is less than 5");
    } else if data > 5 {
        println!("A is greater than 5");
    } else {
        println!("A value is 5");
    }
     
}