fn main(){
let  mut snip = String:: from("This is a snippet of code that I am trying to push to github");

snip.push_str(" and I am trying to see if it will be pushed successfully");
println!("The revised snippet is: {}", snip);
}