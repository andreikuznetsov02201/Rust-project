use std::io;
fn main() {
    // Задаём пустые переменные для пользовательского ввода
    let mut a_str: String = String::new();
    let mut b_str: String = String::new();
    let mut c_str: String = String::new();

    // Добовляем Loop чтобы после завершения подсчета программой, она не завершалась
    loop {
    println!("Решить квадратное уравнение");

    // 1
    println!("Введите a");
    match io::stdin().read_line(&mut a_str) {
        Ok(_) => {},
        Err(e) => {
            println!("Ошибка ввода 1 компонента - {}", e);
        }
    }

    // 2
    println!("Введите b");
    match io::stdin().read_line(&mut b_str) {
        Ok(_) => {},
        Err(e) => {
            println!("Ошибка ввода 2 компонента - {}", e);
        }
    }

    // 3
    println!("Введите c");
    match io::stdin().read_line(&mut c_str) {
        Ok(_) => {},
        Err(e) => {
            println!("Ошибка ввода 3 компонента - {}", e);
        }
    }

    // Приводим к другому типу данных
    let a: f64 = a_str.trim().parse().unwrap();
    let b: f64 = b_str.trim().parse().unwrap();
    let c: f64 = c_str.trim().parse().unwrap();

    // Дискриминант
    let d: f64 = (b * b) - 4.0 * (a * c);

    // Условия для него (2 корня если D > 0; 1 корень если D = 0; н.к. если D < 0)
    if d > 0.0 {
        let x1: f64 = ((-b) + d.sqrt()) / (2.0 * a);
        let x2: f64 = ((-b) - d.sqrt()) / (2.0 * a);

        println!("Решено!\nЕсть 2 корня\nD = {}\nКорень 1 = {}\nКорень 2 = {}", d, x1, x2);
    }

    if d == 0.0 {
        let x: f64 = (-b) / (2.0 * a);

        println!("Решено!\nЕсть 1 корень\nD = 0\nКорень = {}", x);
    }

    if d < 0.0 {
        println!("Нет корней!\nD < 0\nD = {}", d);
    }
}
}
