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
    let mut name: &str = "Alex";
    println!("Name: {}", name);

    // name.push_str(" Test"); // error
    // &str нельзя изменять

    // но так можно name = "Denis"; Но без mut работать не будет. 

    name = "Denis";

    // Вывод 

    println!("Привет, {}", name);


    println!("------------ Практика ------------");
    
    let s1 = String::from("Hello"); // можно менять содержимое строки
    let mut s2: &str = "World"; // нельзя менять содержимое строки
    println!("{}", s2);
    s2 = "Alice";

    println!("{}", s1);
    println!("{}", s2);




    // Методы для работы со строками. 

    // push_str() - добавляет текст в конец строки
    // push - добавляет 1 символ в конец строки 
    // Объединение строк через + 

    let nice = String::from("Привет, сегодня ");
    let nice2 = String::from("отличная погода");

    // Объединение строк
    // let mut res = nice + &nice2;

    // let res_str = nice + &nice2;
    // println!("{}", res_str);

    // Либо использовать format! - более профессиональный способ

    let mut res = format!("{}{}", nice, nice2);


    // Добаляем символ в конец строки
    res.push('!');
    println!("Вывод: {}", res);

    let mut text_title = String::from("Lorem Ipsum is simply dummy text of the printing and typesetting industry.");
    text_title.push_str("Lorem Ipsum has been the industry's standard dummy text ever since 1966, when designers at Letraset and James Mosley, the librarian at St Bride Printing Library in London, took a 1914 Cicero translation and scrambled it to make dummy text for Letraset's Body Type sheets.");
    println!("Сообщение: {}", text_title);

    // Узнаем сколько символов строка
    println!("Строка занимает кол-во {} byte", text_title.len());
}
