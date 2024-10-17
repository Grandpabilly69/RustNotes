fn main() {
    //This is a conditional assignment
    //make sure that both operators are same type
    //if not you will run into errors
    //You can convert type using the "as 'type'" operator
    let cond = 2 > 4;


    //compound conditions
    //this is linked with logical operators
    // and -> &&
    //or -> ||
    //not -> !
    //there is a presedence when applying the operators
    //first: not,     then: and,    then: or
    let food = "cookie";

    //note there is no use of parenthesis
    //this is a standard if statement,
    //and we just add the else onto it
    //fairly self-explanatory
    if food == "cookie" {
        println!("food");
    }else {
        println!("not food");
    }

    //if else statement works same as else,
    //but you can add another condition to check
    //note that it will always work in descending order
    //this means that it starts at the top and
    //works its way down until a condition is met,
    //or it reaches an else statement
    //once a condition is met everything after will not run
    if cond {
        println!("cond");
    }else if !cond {
        println!("not cond");
    }else {
        println!("weird");
    }
}