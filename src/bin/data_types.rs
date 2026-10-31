use std::io;
//use rand::Rng;

fn main(){
// scalar and compund data types
//scalar data types
// integer, floating point, boolean and character
//compound data types
// arrays, tuples, slices and strings

path_scalar();
path_array();
path_tuple();
path_slice();
path_vector();
path_struct();
}

fn path_scalar(){
    // integer
    // number withou decimal component
    // generally of two types signed and unsigned where signed can be negative and unsigned can only be positive
    // signed integers are represented by i8, i16, i32, i64 and i128
    let signed_integer:i32 = -100;
    println!("The signed integer is {}",signed_integer);

    // unsigned integers are represented by u8, u16, u32, u64 and u128
    let unsigned_integer:u32 = 100;
    println!("The unsigned integer is {}",unsigned_integer);
    //the unsigned integer is generally used for counting and indexing and is larger than the signed integer in terms of range
    // additonally there exists the isize and usize types which depend on the architecture of the computer -- mostly used for indexing and counting as well

    // floating point
    // number with decimal component
    // they are of two main types the f32 and f64
    let pi:f32 = 3.142;
    println!(" the value of pi is {pi}");
    // when the floating point is not specified it defaults to f64 which is more precise than f32
    let pi_64:f64 = 3.141592653589793;
    println!(" the value of pi is {pi_64}");
    // boolean
    let x = true;
    println!("{x}");

    

    // character

    // basic numerical operations
    //addition, subtraction, multiplication, division and remainder
    let sum: i32= 5 + 10;
    let difference: f64  = 95.5 - 4.3;
    let product: i32 = 4 * 30;
    let quotient: f64 = 56.7 / 32.2;
    let remainder: i32 = 43 % 5;
    let power: f64 = 2.0_f64.powf(3.0_f64);
    let truncated_division: i32 = -5 / 3;
    println!("The sum is {sum}, the difference is {difference}, the product is {product}, the quotient is {quotient}, the remainder is {remainder}, the power is {power} and the truncated division is {truncated_division}");
    
}

fn path_array(){
    // general array

    // printing out the whole array
    let numbers:[u32; 5] = [20,16, 132 ,268 ,184];
    println!("The numbers are {:?}", numbers);

    // fetching the elements of the array
    println!("the first element is  {}",numbers[0]);
    println!("the fifth element is  {}",numbers[4]);

    // arrays of strings
    let rocket_engine_parts:[&str ; 7] =
    ["convegent-divergent nozzle","combustion chamber", "fuel injector assembly","turbo pump","gas impeller", "turbine","fuel pump"];

    println!("The parts of a rocket engine are {:?}",rocket_engine_parts);

    // initializing a shorthanded array

    let short_hand = [1,2,3,4,5];
    println!("The short hand array is {:?}",short_hand); 
    
    // accessing the shorthand via indexing
    let first_short = short_hand[0];
    println!("The first element of the shorthand array is {}",first_short);
    //can a short hand be accessed directly as a normal array would
    println!("the second element of the shorthand is {}",short_hand[1]);// turns out it can no need to create a new variable in this simple code scenario

    // invalid array access

    let index_test = [1,2,3,4,5,6]; // a  number array
    let animal_index = ["cow","goat","chicken","donkey", "monkey", "zebra"];// an aimal string array


    println!("enter and index to access the array element");

    let mut enter_index = String::new();// creating a new varible to store input as a string


    io::stdin()
    .read_line(& mut enter_index)
    .expect("failed to read the index");// reads input

    let enter_index:usize = enter_index
    .trim()
    .parse()
    .expect("enter a valid index");// converts input to an integer

let element = index_test[enter_index];
let element_2 = animal_index[enter_index];

println!("for the index given ---- {enter_index} the array element is {element} and that in the animal clause will be {element_2}");

// when the array index is larger that the array length the program panics

// initializing an array with same value
let same_value_array = [23; 10];
println!("The array with the same value is {:?}",same_value_array);
    // Differnce btwn debugable format and the display format


}

fn path_tuple(){

    let human=("JOHN",20 ,false); // self listed data types

    println!("The contents of the tuple are --- {:?}", human);

    // calling the data types manually

    let human2:(&str, i32 , bool)=("JOHN",20 ,false);
    println!("The contents of the tuple are --- {:?}", human2);
    
    // notice how john is a string slice rather than an actual string we can directly conver it into  string type 

    let human2:(String, i32 , bool)=("JOHN".to_string(),20 ,false);
    println!("The contents of the tuple are --- {:?}", human2);

    // crazy that tuples can be used with compund data types such as arrays
    let human3:(&str, i32 , bool,[i32;5])=("JOHN",20 ,false,[189,675,400,5646,777]);
    println!("The contents of the tuple are --- {:?}", human3);

    // accessing the elements of the tuple via indexing
    println!("The first element of the tuple is {}",human3.0);
    println!("The second element of the tuple is {}",human3.1);
    println!("The third element of the tuple is {}",human3.2);

    //indexin an array within a tuple

    println!("The elements within the array inside the tuple is {:?}", human3.3);

    // accessing individual elements

    println!("the individual elements are ");
    println!("first element {}",human3.3[0]);
println!("second element in array {}",human3.3[1]);

// unpacking tuple values into individual variables using pattern matchin

let human_3 = ("John",20,false);

let (name , age, is_correct_age) = human_3;

println!("The name is {name}, the age is {age} and the boolean value is {is_correct_age}");


}

fn path_slice(){
    let integer_slice:&[i32;6 ] = &[167,28,490,6,878,56];
    println!("the string slice contains {:?}",integer_slice);
    // accessing the elements of the string slice

    println!("the first element of the slice is {}",integer_slice[0]);
    println!("The second element within the string slice is {}", integer_slice[1]);

    // question can string slices be used with actual strings
    
    let slicing_string:&[&str; 5] = &["cynthia","precious", "micheka", "charity","ngina"];
    
    println!("the string slice cantains the following elements {:?}",slicing_string);
    // i assume accessing the elemens is just the same as before
    

    println!("the string slice cantains the following third element {}",slicing_string[2]);

    // what about the scenario where i want to convert several string slices to Actual strings

    let slice_to_string:[ String; 5] = ["cynthia".to_string(),"precious".to_string(), "micheka".to_string(), "charity".to_string(),"ngina".to_string()];

    println!("The contents of the sliced string data set is {:?}",slice_to_string);

    // the benefit of using strings over string slices is that they are mutable and growable while sring slices are immutable and fixed in size. Additionally, strings can be manipulated and modified, while string slices are read-only views into existing strings.

   // let mut test_push:&[String]= &["chopapi".to_string()];

    let mut push_over = String::from("chopapi");

   push_over.push_str(" munanyo"); // this method works as expected and yields the desired result

    println!("The string after pushing is {}",push_over);
    
   // test_push.push_str&["munanyo"]; // as expected this method was far fetched and didnt yield any reasonable results



// chnanging elements


}
fn path_vector(){
    

}
fn path_struct(){
    
}