

fn main (){

// first am going to go through all the ownership rules one at a time


rule_1();
rule_2();
rule_3();
post_scope();
}

fn rule_1(){

    // Each value in rust has a variable that is its owner



    let s1 = String::from(" chopapi munanyo galore");

    let len = count_length(&s1);

    println!("the length of the string {s1} is {len}");

}

fn count_length(x: & String) -> usize{

    x.len()

}

fn rule_2(){
// there can only be one owner at a time

let s2 = String::from("hello");

/* 
let s3 = s2; ---here  the value of the string is moved from s2 to s3

and therefore s2 no longer owns the value

println!(" the value of the string is {s3}"); --- this part compiles no problem as the value is already in s3

println!(" the value of the string is from the first variable is {s2}");-- program panics as the value no longer exists in s2

we can solve this by cloning the value in s2
*/

let s3 = s2.clone();

println!("The value of the string in both  cases is {s2} and {s3}");
}

fn rule_3(){
    // when the owner goes out of scope the value is dropped

    let s1 = String::from("shawty good");

    let len:usize = check_shawty(& s1);

    println!("the value of the string {s1} is {len}");

}

fn check_shawty(y:& String) -> usize{

    y.len()
}

fn post_scope( /*z: & String*/){
/*  println!("{}",&s1);

-- This function fails to run as s1 is out of scope and the function post scope isnt called within the same
function as s1
*/

}