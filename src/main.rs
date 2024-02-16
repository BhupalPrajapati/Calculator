use std::io;



// Here we can display the menu
fn display_menu(){
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");
    println!("5. Exit");
}

fn main() {
    // simply take user input
loop {
    display_menu();
    // take input from the user
    println!("Enter your choice (1-5):");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");



//  it is the error handling for the match choice which is perform the opearion in the program
    let choice:u32 = match choice.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("Invaild input please make sure you enter vaild input");
            continue;
        }
    };

    // Define a matching pattern fot the match value according to user input
    match choice {
        1=>perform_operation(add),
        2=>perform_operation(subtarct),
        3=>perform_operation(multiply),
        4=>perform_operation(divide),
        5=>{
            println!("Exiting calculator");
            break;
        }
        _=>println!("invaild operations, plz select vaild option(1-5)."),
        
    }
}


}

   
  // this is function which perform as operation take aurgument as a function

fn perform_operation(operation:fn(Vec<f64>)->f64){
println!("Enter Number separated by space :");
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read line");

// paise input numbers
// here we can implement the closure and combinator also iter over the iterators

let numb = input.trim().split_whitespace()
                    .map(|num|num.parse().expect("Invaild number"))
                    .collect::<Vec<f64>>();

 if numb.is_empty(){
    println!("No numbers entered.");
    return;
 }    

 // we stored our numbers 
 let result = operation(numb);
 println!("Result is : {}",result);           
}

// function for add two numbers
fn add(numb:Vec<f64>)->f64{
    numb.iter().sum()
}

// function for the subtraction
// it iterates the over the remaining element in the vector whic is started form the index 1 onwards
// for each subsequent element, it subtract that element from the result
fn subtarct(numb:Vec<f64>)->f64{
    let mut  result = numb[0];
    for &num in &numb[1..]{
        result -= num;
    }
    result
}

// fun for the divide the element 
// Iterates over the entire iterator, multiplying all the elements

fn multiply(numb:Vec<f64>)->f64{
    numb.iter().product()
}

// fun for the divide two number 
// if any element present in function zero then this show msg
fn divide(numb:Vec<f64>)->f64{
    if numb.iter().any(|&x|x==0.0) { 
        println!("Error: can't divide by Zero.");
        return 0.0;
    }
    let mut result = numb[0];
    for &num in &numb[1..]{
        result/=num;
    }
    result
}





