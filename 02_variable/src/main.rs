fn main() {
    // создаем переменную
    let num_code1 = 1234;
    let num_code2 = 5678;
    let num_code3 = 9012;

    let x = 10;
    let y = 20;
    let z = 30;

    
    println!("x = {2}, y = {0}, z = {1}", x, y, z);

    // Аргументы 

    println!("x = {1}, {1}, {1}, {2}, y = {2}, z = {0}", x, y, z);

    let num: i32 = 55;
    let x_num: i32 = 100;

    println!("num = {1}, x_num = {0}", num, x_num);

    // выводим переменную через макрос

    println!("Проверка...\nКод: {}\nПроверка...\nКод: {}\nПроверка...\nКод: {}", num_code1, num_code2, num_code3);

    let (z, y, x) = (100, 50, 30);
    println!("z = {2}, y = {1}, x = {0}", x, y, z); // "z = 100, y = 50, x = 30"

    let (a, b, c, d, e, f) = (5, 10, 15, 20, 25, 30);

    // "a = 5, b = 10, c = 15, d = 20, e = 25, f = 30"
    println!("a = {3}, b = {}, c = {5}, d = {}, e = {4}, f = {}", b, d, f, a, e, c);

    println!("b, d, f, a, e, c");

    // float

    // ------------------------------------------- // 

    println!("----------------------------------\n");


    // float f32 and f64. f64 - по умолчанию 

    let f_x: f64= 2.5; 
    let f_d: f64 = 3.5;

    println!("{} + {} = {}", f_x, f_d, f_x + f_d);


    let x_d: f64 = 5.5;
    let x_w: f64 = 7.1;

    println!("{0} + {1} = {2}", x_d, x_w, x_d + x_w);

    // Неизменяемая строка (&str)

    let name: &str = "Alex";
    println!("{}", name);

    // Изменяемая строка (String)

    let mut hello = String::from("Hello");
    hello.push_str(", world!!!"); // добавляем текст в конец строки
    println!("{}", hello);

    let mut name_user: String = String::from("Ivan");
    name_user.push_str(", Hello");
    println!("{}", name_user);

    let name_user_title = "Hello, progger!!";
    println!("Title: {}", name_user_title);

    
    // shadowing -  затенение/скрытие переменных

    println!("\n-------------------- Shadowing --------------------\n");

    let number = 20;
    println!("Number = {}", number);
    let number = 10;
    println!("Number = {}", number);
    let number = 500;
    println!("Number = {}", number);


    // Task

    let name = "Alex";
    let age: u8 = 20;

    println!("Привет меня зовут {}, мне {} лет", name, age);

    // task 2

    let a = 10;
    let b = 5;

    println!("{} + {} = {}", a, b,  a + b);
    println!("{} - {} = {}", a, b,  a - b);
    println!("{} * {} = {}", a, b,  a * b);
    println!("{} / {} = {}", a, b,  a / b);
    println!("{} / {} = {}", a, b,  a % b);

    
}
