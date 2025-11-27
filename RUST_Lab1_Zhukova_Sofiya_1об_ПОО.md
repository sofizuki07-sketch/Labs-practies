# RUST Лабораторная работа №1
### Задача 1
#### Постановка задачи:
Напишите программу, которая запрашивает у пользователя имя и выводит на экран приветственное сообщение с использованием этого имени.
#### Список идентификаторов:
|Имя переменной|Тип данных|Описание и смысл|
|---|---|------|
|name|String|Переменная с именем пользователя|

#### Код программы:
```Rust
use std::io; //подключаем основную библиотеку ввода вывода

fn main() {
    println!("Пожалуйста, введите ваше имя:");
    
    let mut name = String::new(); // Создаём изменяемую переменную для хранения имени
    
    io::stdin() // Читаем ввод пользователя из стандартного потока ввода (stdin)
        .read_line(&mut name)
        .expect("Ошибка чтения ввода"); // Обрабатываем возможную ошибку ввода
    
    let name = name.trim(); // Удаляем символ новой строки из конца строки
    
    println!("Доброго времени суток, {}!", name);
}
```
#### Результат работы программы:
<img width="496" height="110" alt="4f32a8deb13b92382dac120d29f99de9" src="https://github.com/user-attachments/assets/2d8bd4a8-a34f-41ec-9583-39bee6674b19" />

### Задача 2
#### Постановка задачи:
Создайте переменную типа целое беззнаковое число и выведите ее значение на экран. Явно укажите тип переменной. Затем измените значение переменной и снова выведите его.
#### Список идентификаторов:
|Имя переменной|Тип данных|Описание и смысл|
|---|---|------|
|num|u32|Переменная, в которой изменяем исходное значение|
#### Код программы:
```Rust
fn main() {
    
    let mut num: u32 = 32; // Создаём изменяемую переменную
    println!("Значение num = {}", num);

    num = 64;
    println!("Изменённое значение num = {}", num); 
}
```
#### Результат работы программы:
<img width="508" height="96" alt="image" src="https://github.com/user-attachments/assets/01f071cc-50a0-458f-9bf9-f01cccc58b65" />

### Задача 3
#### Постановка задачи:
Напишите функцию, которая принимает строку и возвращает ее длину (количество символов). Затем вызовите эту функцию с различными строками.
#### Список идентификаторов:
|Имя переменной|Тип данных|Описание и смысл|
|---|---|------|
|s1|&str|Изменяемая строка|
|str|&str|Аргумент функции, строка для подсчёта длины|
|len|usize|Результирующая переменная, длина строки|
#### Код программы:
```Rust

fn main() {
    
    let mut s1 = "Кавабанга"; // Создаём изменяемую переменную 9
    print(s1, str_length(s1));

    s1 = "СкубиДу"; // 7
    print(s1, str_length(s1));

    s1 ="V S Code Is Beautiful!"; //22
    print(s1, str_length(s1));

    s1 = ""; // 5
    print(s1, str_length(s1));
}

fn str_length(str: &str)-> usize {
    str.chars().count()
}

fn print(str: &str, len: usize){
    println!("Строка: {}, длина строки = {}", str, len);
}
```
#### Результат работы программы:
<img width="786" height="146" alt="image" src="https://github.com/user-attachments/assets/db1ba097-0c06-4cfa-a7b1-478cf02f6e40" />

### Задача 4
#### Постановка задачи:
Задайте структуру Car с полями brand, model и year, и создайте несколько экземпляров этой структуры. Выведите информацию о каждой машине на экран.
#### Список идентификаторов:
|Имя переменной|Тип данных|Описание и смысл|
|---|---|------|
|Car|struct|Структура для определения автомобиля|
|num|u32|Поле структуры, переменная для порядкового номера|
|brand|String|Поле структуры, переменная для бренда|
|model|String|Поле структуры, переменная для модели|
|year|u32|Поле структуры, переменная для года выпуска|
|car1|struct Car|Переменная структуры, аргумент функции|
|Roman|struct Car|Переменная структуры, структура под именем владельца машины|
|Alex|struct Car|Переменная структуры, структура под именем владельца машины|
|Stephan|struct Car|Переменная структуры,структура под именем владельца машины|
#### Код программы:
```Rust

struct Car{
    num: u32,
    brand: String,
    model: String,
    year: u32
}

fn print(car1: Car){
    println!("Машина {}: {}  {}  {}г.", car1.num, car1.brand, car1.model, car1.year);
}
fn main() {
    
    let roman = Car{
        num: 1,
        brand: "Ford".to_string(),
        model: "Galaxy".to_string(),
        year: 2014
    };
    let alex = Car{
        num: 2,
        brand: "Chery".to_string(),
        model: "Arrizo 8".to_string(),
        year: 2024
    };
    let stephan = Car{
        num: 3,
        brand: "Volkswagen".to_string(),
        model: "New Beetle".to_string(),
        year: 2011
    };

    print(roman);
    print(alex);
    print(stephan);

}
```
#### Результат работы программы:
<img width="682" height="120" alt="image" src="https://github.com/user-attachments/assets/4129f897-cef1-4c6b-aa6d-9ffc36f91005" />

### Задача 5
#### Постановка задачи:
Напишите программу, которая запрашивает у пользователя число 𝑁 и выводит на экран 𝑁­ное число Фибоначчи. Используйте рекурсию для решения этой задачи.
#### Список идентификаторов:
|Имя переменной|Тип данных|Описание и смысл|
|---|---|------|
|input|String|Вводимое число в виде строки|
|num|u64|Вводимое число, полученное из изначальной строки|
|res|u64|Результирующая переменная, значение num-го члена ряда|
|n|u64||
#### Код программы:
```Rust
use std::io;


fn fib(n: u64) -> u64{
    match n {
        1=> 0,
        2=> 1,
        _=>fib(n-1) + fib(n-2)
    }
}

fn main() {
    
    println!("Введите номер члена ряда:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка чтения числа");

    let num: u64 = match input.trim().parse(){
        Ok(num) =>num,
        Err(_)=> { 
            println!("Ошибка!");
            return;
        }
    };

    let res = fib(num);
    println!("F({}) = {}", num, res);

}
```
#### Результат работы программы:
<img width="478" height="116" alt="image" src="https://github.com/user-attachments/assets/92dc894a-7edb-4898-af55-b252f2ca47f6" />

### Задача 6
#### Постановка задачи:
Реализуйте перечисление DayOfWeek для дней недели. Напишите функцию, которая принимает день недели и возвращает следующий день. Обработайте случаи перехода на следующий день недели, если текущий день – воскресенье.
#### Список идентификаторов:
|Имя переменной|Тип данных|Описание и смысл|
|---|---|------|
|input|String|Входная строка с номером дня недели|
|today|DayOfWeek|Переменная,хранящая введённый день в формате DayOfWeek|
|next|DayOfWeek|Результрующая переменная, хранящая следующий день недели в формате DayOfWeek|
|DayOfWeek|enum|Перечисление дней недели|
|curr_day|DayOfWeek|Параметр функции, текущий день в функции|
|day|DayOfWeek|Параметр функции, ответ для вывода|

#### Код программы:
```Rust
use std::io;


fn main() {
    let mut input = String::new();
    println!("Введите день недели(1-7):");

    io::stdin()
            .read_line(&mut input)
            .expect("Ошибка ввода!");

    let today = match input.trim().parse::<u32>() {
        Ok(1) => DayOfWeek::Monday,
        Ok(2) => DayOfWeek::Tuesday,
        Ok(3) => DayOfWeek::Wednesday,
        Ok(4) => DayOfWeek::Thursday,
        Ok(5) => DayOfWeek::Friday, 
        Ok(6) => DayOfWeek::Saturday,
        Ok(7) => DayOfWeek::Sunday,
        _ => {println!("Ошибка, введите число от 1 до 7!");
              return;}
    };

    let next = next_day(today);

    print(next);

}
    
enum DayOfWeek {
    Monday = 1,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

fn next_day(curr_day: DayOfWeek) -> DayOfWeek {
    match curr_day {
        DayOfWeek::Monday => DayOfWeek::Tuesday,
        DayOfWeek::Tuesday => DayOfWeek::Wednesday,
        DayOfWeek::Wednesday => DayOfWeek::Thursday,
        DayOfWeek::Thursday => DayOfWeek::Friday,
        DayOfWeek::Friday => DayOfWeek::Saturday,
        DayOfWeek::Saturday => DayOfWeek::Sunday,
        DayOfWeek::Sunday => DayOfWeek::Monday

    }
}

fn print(day: DayOfWeek) {
    match day {
        DayOfWeek::Monday => println!(" Завтра -> Понедельник"),
        DayOfWeek::Tuesday => println!(" Завтра -> Вторник"),
        DayOfWeek::Wednesday => println!(" Завтра -> Среда"),
        DayOfWeek::Thursday => println!(" Завтра -> Четверг"),
        DayOfWeek::Friday => println!(" Завтра -> Пятница"),
        DayOfWeek::Saturday => println!(" Завтра -> Суббота"),
        DayOfWeek::Sunday => println!(" Завтра -> Воскресенье")
    }
}
```
#### Результат работы программы:
<img width="542" height="392" alt="image" src="https://github.com/user-attachments/assets/6a42aba1-42f4-414c-9685-60149f7d1f65" />

### Задача 7
#### Постановка задачи:
Создайте структуру Product с полями name, price и category, а также перечисление (enum) Category для категорий товаров. Напишите метод для вывода информации о продукте и ассоциированную функцию для подсчета общей суммы товаров в заданной категории из массива продуктов.
#### Список идентификаторов:

#### Код программы:
```Rust
use std::io;

fn main() {
    let products = [
        Product {
            name: String::from("Телефон"),
            price: 2500,
            category: Category::Electronics,
        },
        Product {
            name: String::from("Футболка"),
            price: 1500,
            category: Category::Clothing,
        },
        Product {
            name: String::from("Хлеб"),
            price: 50,
            category: Category::Food,
        },
        Product {
            name: String::from("Ноутбук"),
            price: 50000,
            category: Category::Electronics,
        },
        Product {
            name: String::from("Джинсы"),
            price: 3000,
            category: Category::Clothing,
        },
    ];
    
    // Выводим информацию о всех товарах
    println!("Информация о всех товарах:");
    for product in &products {
        print_product_info(product);
    }
    
    // Подсчитываем общую сумму для разных категорий
    let electronics_total = total_price_in_category(&products, Category::Electronics);
    let clothing_total = total_price_in_category(&products, Category::Clothing);
    let food_total = total_price_in_category(&products, Category::Food);
    
    println!("Общая стоимость товаров по категориям:");
    println!("Электроника: {:.2} руб.", electronics_total);
    println!("Одежда: {:.2} руб.", clothing_total);
    println!("Еда: {:.2} руб.", food_total);

}
    
struct Product {
    name: String,
    price: u64,
    category: Category
}
enum Category {
    Electronics,
    Clothing,
    Food,
    Books,
    Other,
}

fn print_product_info(product: &Product) {
    let category_name = match product.category {
        Category::Electronics => "Электроника",
        Category::Clothing => "Одежда",
        Category::Food => "Еда",
        Category::Books => "Книги",
        Category::Other => "Другое",
    };
    
    println!("Товар: {}", product.name);
    println!("Цена: {} руб.", product.price);
    println!("Категория: {}", category_name);
    println!("---");
}

fn total_price_in_category(products: &[Product], category: Category) -> u64 {
    let mut total = 0;
    
    for product in products {
        // Сравниваем категории через match
        match (&product.category, &category) {
            (Category::Electronics, Category::Electronics) => total += product.price,
            (Category::Clothing, Category::Clothing) => total += product.price,
            (Category::Food, Category::Food) => total += product.price,
            (Category::Books, Category::Books) => total += product.price,
            (Category::Other, Category::Other) => total += product.price,
            _ => {} // Если категории не совпадают, ничего не делаем
        }
    }
    
    total
}
```
#### Результат работы программы:
<img width="620" height="914" alt="image" src="https://github.com/user-attachments/assets/36101ebf-15d5-4688-aef1-6474f6b262f0" />

### Задача 8
#### Постановка задачи:
Создайте структуру Employee с полями name, position, salary, а также перечисление Position для должностей сотрудников. Напишите функцию, которая принимает вектор  сотрудников и возвращает вектор сотрудников заданной должности.
#### Список идентификаторов:

#### Код программы:
```Rust
use std::io;

fn main() {

    let employees: Vec<Employee> = vec![
        Employee {
            name: String::from("Василий Петров"),
            position: Position::Manager,
            salary: 80000,
        },
        Employee {
            name: String::from("Петр Сидоров"),
            position: Position::Developer,
            salary: 70000,
        },
        Employee {
            name: String::from("Мария Гербова"),
            position: Position::Designer,
            salary: 60000,
        },
        Employee {
            name: String::from("Иван Козлов"),
            position: Position::Developer,
            salary: 75000,
        },
        Employee {
            name: String::from("Ольга Смирнова"),
            position: Position::Analyst,
            salary: 65000,
        },
        Employee {
            name: String::from("Алексей Волков"),
            position: Position::Manager,
            salary: 25000,
        },
    ];

    println!("Введите должность (Менеджер, Разработчик, Дизайнер, Аналитик):");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)
               .expect("Ошибка ввода!");
    
    let x = match string_to_position(input.trim()) {
        Some(pos) => pos,
        None => {
            println!("Ошибка: неизвестная должность '{}'", input);
            println!("Доступные должности: Менеджер, Разработчик, Дизайнер, Аналитик");
            return;
        }
    };

    let res = sort_by_position(employees, x);
    
    // Выводим информацию о введённой должности:
    println!("Сотрудники с должностью: {}", input);
    for employee in res {
        print_employee_info(&employee);
    }
}
    
struct Employee {
    name: String,
    position: Position,
    salary: u32,
}

enum Position {
    Manager,
    Developer,
    Designer,
    Analyst,
}

fn string_to_position(input: &str) -> Option<Position> {
    match input.to_lowercase().as_str() {
        "менеджер" => Some(Position::Manager),
        "разработчик" => Some(Position::Developer),
        "Дизайнер" => Some(Position::Designer),
        "Аналитик" => Some(Position::Analyst),
        _ => None, // Если ввод не распознан
    }
}

fn sort_by_position(employees: Vec<Employee>, position: Position) -> Vec<Employee>{
    let mut result: Vec<Employee> = Vec::new();
    for employee in employees{
        match (&employee.position, &position){
            (Position::Manager, Position::Manager) => result.push(employee),
            (Position::Developer, Position::Developer) => result.push(employee),
            (Position::Designer, Position::Designer) => result.push(employee),
            (Position::Analyst, Position::Analyst) => result.push(employee),
            _ => {}
        }
    }
    result
}


fn print_employee_info(employee: &Employee) {
    let position_name = match employee.position {
        Position::Manager => "Менеджер",
        Position::Developer => "Разработчик",
        Position::Designer => "Дизайнер",
        Position::Analyst => "Аналитик"
    };
    
    println!("Сотрудник: {}", employee.name);
    println!("Должность: {}", position_name);
    println!("Зарплата: {} руб.", employee.salary);
    println!("---");
}
```
#### Результат работы программы:
<img width="980" height="424" alt="image" src="https://github.com/user-attachments/assets/ef4f1b83-b70f-47cb-b92a-6d0ccc973bc5" />


###Выполнила: Жукова София 1об ПОО



