fn main() {
    println!("---------------- STRING ----------------\n");

    // неизменяемая строка
    let str_text = String::from("Hello String");
    println!("String output: {}", str_text);

    str_text.push_str(" - Hell"); // error 

    println!("String output: {}", str_text);

    
}
