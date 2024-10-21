//Stack and heap are a part of RAM
//variables take up memory and we need
//to manage it so we don't use it up
//unnecessarily

//stacks use the FILO principle
//First In Last Out
//we try use as much stack as possible
//then we use heap
//since rust is faster with stack

fn main(){
    //x is allocated to a memory address
    //When the scope ends x is automatically
    //removed from memory
    //The stack cannot store dynamic variables
    //It can only hold variables that cannot change
    let x = 2;
    let y = x;

    example();
    //this creates copies of these variables
    //in memory and the function knows to
    //use the 2 copies of the vars because
    //it is at the top of the stack
    add(x,y);

    //The heap stores dynamic variables
    //that is why we make variables mutable
    //if they are mutable they get allocated to heap

    heap();

}

fn example(){
    let a = "true";
    let b = false;
}

fn add(x: i32, y: i32) -> i32{
    x + y
}

fn heap(){
    let x = 2;
    //the literal string is immutable and goes to the stack
    //String::from makes the string dynamic so we
    //can add and remove characters
    //this means that it will go to the heap

    //when storing in the heap it looks for a space
    //that is large enough to store the string
    //the str variable goes on the stack
    //and acts as a pointer for the literal string
    //this means that str will store the memory address of the literal
    let str = String::from("hello world");
}

//in conclusion, you will want to try
//store as many variables into the stack
//than the heap since the heap
//takes longer to store and retrieve data