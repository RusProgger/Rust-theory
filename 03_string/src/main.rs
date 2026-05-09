fn main() {
    println!("---------------- STRING ----------------\n");

    // String — владеет строкой
    // строку можно изменять, если добавить mut
    let str_text = String::from("Hello String");
    println!("String output: {}", str_text);

    // push_str() добавляет текст в конец строки
    // str_text.push_str(" - Hell"); // error: str_text is not mutable

    // println!("String output: {}", str_text);


    // &str — строковый срез (ссылка на строку)
    // обычно используется только для чтения
    let name: &str = "Alex";
    println!("Name: {}", name);

    // name.push_str(" Test"); // error
    // &str нельзя изменять

    // но так можно name = "Denis"; Но без mut работать не будет. 

    name = "Denis";

    // Вывод 

    println!("Привет, {}", name);
}
