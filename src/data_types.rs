fn main(){
    //primitive data types
    //Rust has 2 types of primitive types
    //Scalar and Compound
    //Scalar is something with a single value like int or bool
    //compound is something with multiple values
    //like arrays and tuples
////////////////////////////////////////////////////////
    //Scalar
    /////////////////////////////////////////////////////
    //this is a signed integer of 32 bits
    //also default value in rust
    //size of bits for ints 8, 16, 32, 64, 128
    let x: i32 = 4;

    //unsigned ints are the same as normal integers
    //but cannot have a negative
    let y: u32 = 5;

    //floating point values
    //we have 2 floating point types
    //f32 -> single precision
    //f64 -> double precision
    //default type id f64

    let float_point: f32 = 1.0;
    let fl_pt: f64 = 3.34;

    //boolean can only be true or false
    //note bools are all lower case
    //note you can also use 0 or 1
    //to represent false or true respectively
    let b:bool = false;


    //character/char type
    //note chars only use single quotes

    let letter:char = 'A';
    ///////////////////////////////////////////////////////
    //compound types

    //tuples
    //tuples are different types based on the values
    //in the first parenthesis
    //tupples are imutable by default just like the other variables
    //tupples cannot be formatted by default println!
    let mut tup:(i32, bool, char) = (1, true, 'a');

    //you need to specify the index of the tuple to print
    //this will print true since it is the second item
    println!("{}",tup.1);

    //you can also change different indeces as seen below
    //and make sure the tuple is mutable
    tup.0 = 10;

    //you can also change the whole tuple as seen below
    tup = (23, false, 'b');

    //arrays
    //declaring arrays are the same/similar to java
    //arrays are immutable by default
    //you must make it mutable to edit like
    //every other type
    //arrays have a fixed length and type
    //this is how to statically assign the type
    //first argument is the type
    //second argument is the length
    let mut arr:[i32;5] = [1,2,3,4,5];

    //not you dont have to instantly initailize array
    //note that the array still has to be
    //intialised before you can use it
    let arr2: [i32;5];

    //changing array values are the same/similar to java
    arr[3] = 22;

    /////////////////////////////////////////////////////
    //final notes
    //if you try the below:
    let z = y;
    //it auto assigns the same u32 in this case since you cannot
    //convert them to different types
    //therefore the z var inherits the data type of y
    //if you try to convert data types you will get an error
}