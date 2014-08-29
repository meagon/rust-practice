pub fn public_function (){
    println!("called erty's `public_function()`");
}
fn private_function(){
    println!("called erty's private_function()");
}
pub fn indirect_access(){
   print!("called erty's in that <\n>");
   private_function();
}
