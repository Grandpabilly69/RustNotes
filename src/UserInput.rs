//to get user input we need to import the input output libary
//the use key word is for import
//std is for standard
//we are basically importing the standard crate
//note that crate is the same as package
//the "::" is the same as "." in other languages
//it is the path seporator operator
use std::io;

fn main() {
    println!("name: ");
    //we need to declare the name
    //make sure it is mutable and we
    //assign the value of string type
    let mut name = String::new();

    //stdin stands for standard input
    //&mut name is creating a mutable reference to name
    //the readline directly modifies the data stored in
    //the variable name through pointers
    //the expect handles and caches the error
    io::stdin().read_line(&mut name).expect("Failed to read name");

    //outputting what user entered
    println!("{}",name);
}