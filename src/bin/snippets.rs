use std::io;

fn main(){
// this specific code base is to write code snippets that give  me a general idea of how to use rust programming language and ow its programs work

// found code that takes multiple inputs and stores them in tuple

//multi_input();

calc_len()

}
/*fn multi_input(){

println!("Enter Name, Age and Height E.g (Sarah, 10 , 1.6)");

let mut answers = String::new();

io::stdin().read_line(& mut answers). expect("failed to fetch input");


let parts = answers.trim().split_whitespace();



} */

fn calc_len(){
    
    let mut  hulu = String::from("I am the king of the world");

    let len = check_hulu(& mut hulu);

    println!("The length of the string {hulu} is {len}");


}

fn check_hulu(x :& String)-> usize{

    x.len()
}
