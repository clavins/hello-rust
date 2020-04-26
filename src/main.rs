//filename: src/main.rs

//Define a global constant.
const MAX_NUM: u32 = 1000;

//Define a rust function. The mian() function is always the first code that runs in every executable Rust program.
fn main() {    
    
    //Define a const
    const MAX_POINTS: u32 = 100;
    println!("Max points is {}.", MAX_POINTS);      //100
    println!("Max number is {}", MAX_NUM);          //1000

    //Define a immutable (default) variable x, you can't assign value twice to a immutable variable.
    let x = 5;
    println!("The value of x is: {}.", x);          //The value of x is: 5.

    /*
    Differences between immutable variables and consts: 
      (1) You aren’t allowed to use mut with constants.
      (2) For constants, the type of the value (u32) must be annotated.
      (3) Constants can be declared in any scope, including the global scope
    */

    //Define a mutable variable y, you can assign new value to it.
    let mut y = 10;
    println!("The old value of y is {}.", y);       //The old value of y is 10.
    y = 20;
    println!("The new value of y is {}.", y);       //The new value of y is 20.

    //Shadowing - Define a new varible with the same name as the old one.
    let t = 8;          //Define variable t and set it's value 8;
    let t = t + 1;      //Define a new variable t; 
    let t = t + 2;      //Define another new varible t;
    println!("The value of t is {}.", t);           //The value of t is 11.

    let spaces = "    ";
    let spaces = spaces.len();  //Since we are defining a new variable spaces, we can change it's type.
    println!("The value of spaces is {}.", spaces);//The value of spaces is 4.

    //Data Type - Rust is a statically typed language, every value in Rust is of a certain data type and it must know the types of all variables at compile time.
    /*  Scalar types - single value types, Rust has four primary scalar types: 
        (1) integers: 
            - Signed: i8, i16, i32, i64, i128
            - Unsigned: u8, u16, u32, u64, u128 
        (2) floating-point numbers: f32, f64
        (3) Booleans: bool
        (4) characters: char
    */

    //integers
    let n1: i32 = 99_222;
    println!("n1={}",n1);
    let n2: u64 = 0xff;
    println!("n2={}",n2);
    let n3: u16 = 0o77;
    println!("n3={}",n3);
    let n4: i16 = 0b11100110;
    println!("n4={}",n4);
    let n5: u8 = b'A';          //only for u8
    println!("n5={}",n5);
    
    //float numbers
    let f1 = 3.0;
    let f2: f64 = 5.67;
    println!("Resut is {}.", f1*f2);

    //boolean
    let is_done = false;
    if !is_done {
        println!("Please hurry!");
    }

    //char
    let c1 = 'a';
    let c2 = 'b';

    //Compound Types - group multiple values into one type.
    /*
        Tuple - grouping values with a variety of types        
    */    
    let circle: (u16, f32, f64) = (2, 12.566, 50.265);
    println!("circle.0 is {}.", circle.0);
    println!("circle.1 is {}.", circle.1);
    println!("circle.2 is {}.", circle.2);

    let (r, p, s) = circle;
    println!("Area of circle is {}.", s);    
    
    /*       
        Array - collection of values with the same type
    */    
    let weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    for i in 0..weekdays.len() {
        println!("{}", weekdays[i]);
    }

    //Define data type and length of an Array in the square brackets [datatype; length].
    let nums: [u32; 5] = [1, 2, 3, 4, 5];
    println!("The first number of nums is {}.", nums[0]);

    //Define an array with the same value for each element.
    let ages = [10; 5];         //ages = [10, 10, 10, 10, 10]
    println!("The last age is {}.", ages[4]);


}
