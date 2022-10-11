fn main() {
    let variable:u8 = 128;
    println!("variable: {}, size: {} in bytes", variable, std::mem::size_of_val(&variable)); //access to variable size in bytes, passing variable memory

    let variable2 = 300; //no type declaration assumes a default value, i32 in this case
    println!("variable2: {}, size: {} in bytes", variable, std::mem::size_of_val(&variable2));

    let mut boolean: bool = false; //mut allow variables to be mutable. By default, they're not
    boolean = true; //note the warning by the compiler
    println!("boolean: {}, size: {} in bytes", boolean, std::mem::size_of_val(&boolean));
}