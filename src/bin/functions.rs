use std::io;

// the main function is the entry point of the program and is where the execution of the program begins

fn another_main_function(){
    // this is to show that rust doesnt care what function is written first it only cares who the caller is and that is the main funcition
    println!("This is another main function.");
}
fn main(){
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