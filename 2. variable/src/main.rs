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
    
}
