use std::io;

fn main(){
    println!("enter numbers 20 to 30 numbers");
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    ;
    
    
   
    let change_input =
    input.trim()
    .parse::<i32>()
    .expect("please type a number, Num lx");


    let check_type = check(&change_input);
   

}

fn check(typ:&i32) ->i32
 {
    if typ % 2 == 0 {
        println!("{} is even number", typ);
    } else {
        println!("{} is odd number", typ);
    }
    return *typ;
 }