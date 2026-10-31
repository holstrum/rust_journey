use std::io;

// the main function is the entry point of the program and is where the execution of the program begins

fn another_main_function(){
// this is to show that rust doesnt care what function is written first it only cares who the caller is and that is the main funcition
println!("This is another main function.");
}
fn main(){
// apparently you can also associate variables with functions

let simple_arithmetic = add(5,6);

println!("The simple arithemetic variable to function gives {} ", simple_arithmetic);

// testing input 
println!("What is your name boy");
let mut name = String::new();

io::stdin()
.read_line(& mut name)
.expect("failed to fetch name");

let cleaned = name.trim(); // .trim() is used to remove any leading or trailing whitespace from the input string "\n" or "\r\n"


println!(" How old are you boy");
let mut age = String::new();

io::stdin()
.read_line(& mut age)
.expect("failed to fetch your age");

let given_age:usize = age
.trim()
.parse()
.expect("Incorect data types please give name as an integer value "); 


println!("And how tall are you in meters");
let mut height = String::new();


io::stdin()
.read_line(& mut height)
.expect("failed to fetch name");

let given_height:f64 = height
.trim()
.parse()
.expect("failed!! incorect data type name should be a float");


input_conditions(cleaned,given_age, given_height);


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

// testing out whether i can asign values to functions from user inputs

fn input_conditions(name:&str,given_age:usize,given_height:f64){

println!("So lemme get this straight --------- ######");

println!(" Your names are  {} ,  you are {} years old and are  {} meters tall? ", name, given_age , given_height);


}