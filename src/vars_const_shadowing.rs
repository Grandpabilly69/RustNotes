// rust main function
// note use of fn instead of func like go
fn main() {
    //multiple ways of defining variables
    //this an implicet way of assigning type
    //meaning the compiler decides what the type
    //is based on first input
    let mut x = 4;
    println!("The value of x is {}", x);

    //you can create your own scope in rust using {}
    //in this case x = 2 only inside of scope
    //meaning that x will revert back to 4 once the
    //programming leaves the scope
    //note you can also use anything from the exterior scope
    //in the interior scope
    {
        let x = x - 2;
        println!("The value of x is {}", x);
    }

    //note that by default all variables are imutable in rust
    //the key word mut means it is muatable and can change
    //note if you do not want to make variable mutable
    //you can redeclare it
    x = x + 1;

    //this is an explicite way of assigning a
    //type to a variable
    let y :i32 = 5;
    println!("The value of y is {}", y);
    //this is a formatted string and how it is structured
    //note is very similar to python
    //note bang/! means we are running a macro

    //you can also redefine variables types
    //when you redeclare them
    let y = "hello world";
    println!("The value of y is {}", y);
    println!("{}", x);

    //constants make use of the const key word
    //note that it must be in snake casing with all capitals
    //as shown below
    //a constant cannot change its type or value throughout
    //the whole programme
    //note you must also declare the data type and value
    const SECONDS_IN_MINUTE: i32 = 60;
    println!("{}", SECONDS_IN_MINUTE);
}