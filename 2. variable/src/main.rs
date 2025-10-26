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
    let x_w: f64 = 7.2;

    println!("{0} + {1} = {2}", x_d, x_w, x_d + x_w);

    // Неизменяемая строка (&str)

    

    
}
