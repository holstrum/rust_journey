use std::io;

// the main function is the entry point of the program and is where the execution of the program begins

fn another_main_function(){
    // this is to show that rust doesnt care what function is written first it only cares who the caller is and that is the main funcition
    println!("This is another main function.");
}
fn main(){
// apparently you can also associate variables with functions

let simple_arithmetic = add(5,6);

    another_main_function();
    // a function can be given an argument as a parameter and the function can use that argument to perform some operation or calculation

    my_height(156);
    credentials("Stephen","Rukwaro", 20 , 1.8);
   
}
fn my_height(height:i32){

    println!("my height is {} cm",height);
}

fn credentials(fist_name:&str,scnd:&str,age:i32,height:f64){

    println!("who am I ?");
    println!("I am.......");
    println!("{fist_name} {scnd}, am {age} years old {height} M tall");




}

//Expressions vs statements
// In rust expressions are any lines of code that returns a value while a statement is one that doesnt
// Expressions 
//---------
// booleans
// arithmetic operations
// code blocks {(code)}
//if coditions that return values

fn add(a:i32,b:i32)->i32{

    a+b // an expression that returns a value tied to the given function
}