

fn main (){

// first am going to go through all the ownership rules one at a time

//hardcoded strign literalls--- saved on stack , fast and are immutable as a result

let literal = "lemme see if i can change this";

let new_literal = literal;


println!("{literal}");

println!("the value of the new literal is {new_literal} and the old one is {literal}");

// data on stack on stack is easy to copy and us a result most ownership rules dont apply same case goes for integers

// lemme try not calling let and  see how the hardcoded string literal behaves
/*

trial = literal;

println!("trial gives{trial}"); 

apparently let is a binding that cannot be ignored 
its also the same case for integers */


let x = 90;
let y = x;


println!("The value of x and y are {x} and {y}");

// checking whether let is necessary for integers

let  x = 67;
let y = x + 1;

println!("The value of y is {y}");

// apparently its unnecessary but mutation is required

// the kind of string that can be mutated and modified is the "String type"

let mut modif = String::from("chopapi");

let z = & mut  modif;
z.push_str(", munanyo");

println!("{z}");
// we can also use the clone method on itegers its just completely unneccesary

let z = 56 + 120 ;
let t = z.clone();

// clone is a deep copy method that copies both pointer and memory data
// when moving a value to another string only the pointer , len and capacity is copied 
// This avoids calling duplicated memory

println!("The value of t is {t}");
rule_1();
rule_2();
rule_3();
post_scope();

// you can pass a value to a function just the same way that assignment does

let  godz = String::from("Mecha");

new_owner(godz);

// trying owneship with tuples
let zawarudo = String::from("Totio tameri");
let (zawarudo_len , len ) = calc(zawarudo);

let remain =  len % 9;

let time_stop = len - remain;

println!("the length of {zawarudo_len} is {time_stop} seconds");

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

fn new_owner(own_again:String) -> String{

    println!("{own_again}");

    
    own_again

}

//ownership of variables follows the same pattern each time
// when a variable goes out of scope the drop function is called for heap data
// 

fn calc(s: String)->(String, usize){
    let length = s.len();
    (s, length)
}