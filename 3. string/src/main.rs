fn main() {
    println!("---------------- STRING ----------------\n");

    // неизменяемая строка
    let str_text = String::from("Hello String");
    println!("String output: {}", str_text);

    // Функция push_str() - добачляем текст в конец строки
    // str_text.push_str(" - Hell"); // error not mut

    // println!("String output: {}", str_text);


    // строка только для считывания
    let name: &str = "Alex";
    println!("Name: {}", name);

    // Не стработает push_str() - error
}
