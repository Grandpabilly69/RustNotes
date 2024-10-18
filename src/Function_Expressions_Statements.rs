fn main(){
    //calling function is same as other languages
    //just put the name
    test();
    add_nums(3,5);

    //this a statement
    //they don't return anything
    let x = 20;

    //this is an expression
    let number = {
        let y = 3;
        //note that we don't put ; after y+1
        //because we want it to return to number
        //if we had the ; it would not return the value
        y+1
    };

    let z = add_nums(3, number);
    println!("{:?}", z);
}

//When creating functions you should use snake case
//not Camel case
fn test(){
    println!("Test called");
}

//make sure you specify the type of the parameters as below
fn add_nums(a:i32,b:i32){
    //this displays the 2 numbers and adds them
    println!("{} + {} = {}",a,b,a+b);
}

//note the -> is the return statement sign which changes the function
//into an expression
//note that there is no key word return used
//in rust it will read the last statement and take it as the return
//also note there is no use of the ; in the function
//since we need to return the statement
fn add_returns(a:i32,b:i32) -> i32{
    //there are 2 ways of returning statements
    //this is one with no semicolon or return
    a + b
    //the other is below
    //using return and ;
    //return a+b;
}