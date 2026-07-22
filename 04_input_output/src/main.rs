use std::io;
use std::io::Write;

fn main() {
    print!("Пожалуйста введите ваше имя: ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Ошибка чтения строки");


    println!("Вывод {}", user_input);
}
