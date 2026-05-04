fn main() {
    println!("---------------- STRING ----------------\n");


    let str_text = String::from("Hello String");
    println!("String output: {}", str_text);

    str_text.push_str(" - Hell");

    println!("String output: {}", str_text);
}
