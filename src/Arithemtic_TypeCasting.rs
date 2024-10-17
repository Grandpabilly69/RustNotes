use std::io;
fn main() {
    let x: u8 = 9;  //range 0 - 255
    let y: i8 = 10; //range -128 - 127
   //the below cannot accor since they are considered different types
 // let z = x + y;

    //you can convert to the following though
    //to do the arithmetic
    let z = (x as i8) + y;

    let p: u8 = 255;
    let d: u8 = 1;

    //dividing 2 ints will output an int and automatically
    //truncate the decimal
    let mut e = p / d;
    println!("{}", e);

    //this is the mod operator
    //it works the same as java
    //it gives the remainder after divding the numbers
    e = p % d;
    println!("{}", e);

    //you can also tell rust to treat vars as certain
    //types as shown below
    let f = 55.0f32;
    //you can also use an underscore and it will do the same thing
    let k = 66_i8;
    //you can also use the as key word
    let j = 88 as i8;

////////////////////////////////////////////////////////////////////////
    //converting string to int

    //getting user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    //trim uses the nextline character
    //parse tries to convert the string into
    //the integer type
    //unwrap returns the actual int type
    let int_input: u8 = input.trim().parse().unwrap();

    println!("{}", int_input);
}