# RUST Лабораторная работа №2
## Тема: Ссылки, Владение (Ownership). Реализация методов (Impl). Трейты (Traits). Обобщенные типы (Generics).

### Задача 1
#### Постановка задачи:
Создайте структуру `Pair<T>`, которая хранит два значения одного типа `T`. Реализуйте методы для получения и изменения значений пары. Убедитесь, что использование ссылок и копирование данных работает корректно.

#### Список идентификаторов:
|Имя переменной|Тип данных|Описание и смысл|
|---|---|------|
|`int_pair`|`Pair<i32>`|Экземпляр структуры для типа `i32`|
|`str_pair`|`Pair<String>`|Экземпляр структуры для типа `String`|
|`val1_ref`|`&i32`|Неизменяемая ссылка на первое значение|
|`val2_ref`|`&mut String`|Изменяемая ссылка на второе значение|
|`T`|Обобщенный тип|Параметр типа для структуры `Pair`|

#### Код программы:
```Rust
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }

    fn get_first(&self) -> &T {
        &self.first
    }

    fn get_second(&self) -> &T {
        &self.second
    }

    fn set_first(&mut self, new_first: T) {
        self.first = new_first;
    }

    fn set_second(&mut self, new_second: T) {
        self.second = new_second;
    }
}

fn main() {
    let mut int_pair = Pair::new(10, 20);
    println!("Начальная пара целых чисел: {}, {}", int_pair.get_first(), int_pair.get_second());

    let val1_ref = int_pair.get_first();
    let _val2_ref = int_pair.get_second();
    println!("Первое значение через ссылку: {}", val1_ref);

    int_pair.set_first(30);
    int_pair.set_second(40);
    println!("Измененная пара: {}, {}", int_pair.get_first(), int_pair.get_second());

    let mut str_pair = Pair::new(String::from("Привет"), String::from("Мир"));
    println!("Начальная пара строк: {}, {}", str_pair.get_first(), str_pair.get_second());

    let second_val = str_pair.get_second();
    println!("Второе значение строки: {}", second_val);

    str_pair.set_second(String::from("Rust"));
    println!("Обновленная пара строк: {}, {}", str_pair.get_first(), str_pair.get_second());
}
```
#### Результаты выполнения:
<img width="644" height="256" alt="image" src="https://github.com/user-attachments/assets/e57288be-bcee-4bdb-9622-fa907cb04b67" />

### Задача 2
#### Постановка задачи:
Определите трейт `PrintInfo`, который содержит метод `print_info(&self)`. Реализуйте этот трейт для структуры `Person`, чтобы можно было вызвать метод `print_info` на объекте `Person`.

#### Список идентификаторов:
|Имя переменной|Тип данных|Описание и смысл|
|---|---|------|
|`person`|`Person`|Экземпляр структуры `Person`|
|`name`|`String`|Поле структуры, имя человека|
|`age`|`u32`|Поле структуры, возраст человека|

#### Код программы:
```Rust
trait PrintInfo {
    fn print_info(&self);
}

struct Person {
    name: String,
    age: u32,
}

impl PrintInfo for Person {
    fn print_info(&self) {
        println!("Имя: {}, Возраст: {}", self.name, self.age);
    }
}

fn main() {
    let person = Person {
        name: String::from("Мария"),
        age: 25,
    };
    person.print_info();
}
```
#### Результаты выполнения:
<img width="426" height="70" alt="image" src="https://github.com/user-attachments/assets/da898f9b-b5c8-4cdc-b025-7b0a1a8abf81" />

### Задача 3
#### Постановка задачи:
Разработайте программу для обработки списка задач (Todo List). Создайте структуру `Task` с полями `id`, `title` и `completed`, а также методы для добавления, удаления и изменения статуса задачи. Реализуйте вывод информации о всех задачах.

#### Список идентификаторов:
|Имя переменной|Тип данных|Описание и смысл|
|---|---|------|
|`todo_list`|`TodoList`|Экземпляр структуры, управляющий списком задач|
|`tasks`|`Vec<Task>`|Вектор, хранящий задачи списка|
|`task`|`Task`|Экземпляр структуры, описывающий одну задачу|
|`id`|`u32`|Уникальный идентификатор задачи|
|`title`|`String`|Название задачи|
|`completed`|`bool`|Статус выполнения задачи|

#### Код программы:
```Rust
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, title: String) {
        let id = self.tasks.len() as u32 + 1;
        let new_task = Task {
            id,
            title,
            completed: false,
        };
        self.tasks.push(new_task);
    }

    fn remove_task(&mut self, id: u32) {
        self.tasks.retain(|task| task.id != id);
    }

    fn toggle_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = !task.completed;
                break;
            }
        }
    }

    fn print_all(&self) {
        println!("Список задач:");
        for task in &self.tasks {
            let status = if task.completed { "[x]" } else { "[ ]" };
            println!("{} {}: {}", status, task.id, task.title);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    todo_list.add_task(String::from("Купить молоко и сыр"));
    todo_list.add_task(String::from("Написать лабораторную работу"));
    todo_list.add_task(String::from("Убраться в комнате"));

    todo_list.print_all();

    println!("\nОтмечаем задачу 1 как выполненную:");
    todo_list.toggle_task(1);
    todo_list.print_all();

    println!("\nУдаляем задачу 2:");
    todo_list.remove_task(2);
    todo_list.print_all();
}
```
#### Результаты выполнения:
<img width="642" height="600" alt="image" src="https://github.com/user-attachments/assets/5389d9e4-7a3f-4a9a-8c1c-ac4e1ce7fc4f" />

### Задача 4
#### Постановка задачи:
Напишите функцию `find_unique_elements`, которая принимает вектор элементов типа `T` и возвращает вектор уникальных элементов. Определите трейт `Unique` с методом `is_unique`, который проверяет, является ли элемент уникальным.

#### Список идентификаторов:
|Имя переменной|Тип данных|Описание и смысл|
|---|---|------|
|`numbers`|`Vec<i32>`|Входной вектор целых чисел|
|`words`|`Vec<String>`|Входной вектор строк|
|`unique_numbers`|`Vec<i32>`|Вектор, содержащий уникальные числа|
|`unique_words`|`Vec<String>`|Вектор, содержащий уникальные строки|
|`element`|`&T`|Ссылка на элемент в процессе проверки|

#### Код программы:
```Rust
trait Unique {
    fn is_unique(&self, slice: &[Self]) -> bool
    where
        Self: PartialEq;
}

impl<T: PartialEq> Unique for T {
    fn is_unique(&self, slice: &[T]) -> bool {
        slice.iter().filter(|&x| x == self).count() == 1
    }
}

fn find_unique_elements<T: PartialEq + Clone>(vec: &[T]) -> Vec<T> {
    let mut unique = Vec::new();
    for element in vec {
        if element.is_unique(vec) {
            unique.push(element.clone());
        }
    }
    unique
}

fn main() {
    let numbers = vec![1, 2, 2, 3, 4, 4, 5];
    let unique_numbers = find_unique_elements(&numbers);
    println!("Уникальные числа: {:?}", unique_numbers);

    let words = vec![
        String::from("привет"),
        String::from("мир"),
        String::from("привет"),
        String::from("rust"),
    ];
    let unique_words = find_unique_elements(&words);
    println!("Уникальные слова: {:?}", unique_words);
}
```
#### Результаты выполнения:
<img width="552" height="92" alt="image" src="https://github.com/user-attachments/assets/5b539ca0-7625-46a7-82bf-058288d81410" />

### Задача 5
#### Постановка задачи:
Напишите функцию `sum_collection`, которая принимает коллекцию элементов типа `T` и возвращает их сумму. Ограничьте тип `T` требованием наличия метода `addition()` в некотором трейте `Addable` для поддержки суммирования. Просуммируйте затем любую коллекцию структур вида: `struct Number { value: u32 }`

#### Список идентификаторов:
|Имя переменной|Тип данных|Описание и смысл|
|---|---|------|
|`numbers`|`Vec<Number>`|Вектор структур `Number` для суммирования|
|`total`|`u32`|Результат суммирования|
|`num`|`&Number`|Ссылка на элемент коллекции в цикле|

#### Код программы:
```Rust
trait Addable {
    fn addition(&self, other: &Self) -> Self;
}

struct Number {
    value: u32,
}

impl Addable for Number {
    fn addition(&self, other: &Self) -> Self {
        Number {
            value: self.value + other.value,
        }
    }
}

fn sum_collection<T: Addable + Default>(collection: &[T]) -> T {
    let mut sum = T::default();
    for item in collection {
        sum = sum.addition(item);
    }
    sum
}

impl Default for Number {
    fn default() -> Self {
        Number { value: 0 }
    }
}

fn main() {
    let numbers = vec![
        Number { value: 10 },
        Number { value: 20 },
        Number { value: 30 },
    ];
    let total = sum_collection(&numbers);
    println!("Сумма коллекции: {}", total.value);
}
```
#### Результаты выполнения:
<img width="376" height="56" alt="image" src="https://github.com/user-attachments/assets/3389bdbe-7f0a-455e-aa71-3f694d905011" />

### Задача 6
#### Постановка задачи:
Создайте обобщенную структуру `Collection<T>`, которая содержит вектор элементов типа `T`. Реализуйте методы `add_element()`, `get_element()` и `remove_element()`, ограничив тип `T` трейтом `Clone`.

#### Список идентификаторов:
|Имя переменной|Тип данных|Описание и смысл|
|---|---|------|
|`int_collection`|`Collection<i32>`|Коллекция целых чисел|
|`str_collection`|`Collection<String>`|Коллекция строк|
|`index`|`usize`|Индекс элемента для получения или удаления|
|`element`|`T`|Добавляемый элемент или полученный элемент|

#### Код программы:
```Rust
struct Collection<T: Clone> {
    items: Vec<T>,
}

impl<T: Clone> Collection<T> {
    fn new() -> Self {
        Collection { items: Vec::new() }
    }

    fn add_element(&mut self, element: T) {
        self.items.push(element);
    }

    fn get_element(&self, index: usize) -> Option<T> {
        if index < self.items.len() {
            Some(self.items[index].clone())
        } else {
            None
        }
    }

    fn remove_element(&mut self, index: usize) -> Option<T> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }
}

fn main() {
    let mut int_collection = Collection::new();
    int_collection.add_element(1);
    int_collection.add_element(2);
    int_collection.add_element(3);

    println!("Элемент с индексом 1: {:?}", int_collection.get_element(1));
    println!("Удаляем элемент с индексом 0: {:?}", int_collection.remove_element(0));
    println!("Попытка получить элемент с индексом 5: {:?}", int_collection.get_element(5));

    let mut str_collection = Collection::new();
    str_collection.add_element(String::from("Rust"));
    str_collection.add_element(String::from("is"));
    str_collection.add_element(String::from("awesome"));

    println!("Элемент с индексом 2: {:?}", str_collection.get_element(2));
}
```
#### Результаты выполнения:
<img width="716" height="182" alt="image" src="https://github.com/user-attachments/assets/03ebb642-2490-4b3e-8b9c-d80af4df4644" />

### Задача 7
#### Постановка задачи:
Создайте трейт `Summable`, который требует реализации метода `sum(&self) -> u32` для типов, поддерживающих сложение. Реализуйте этот трейт для структуры `Pair<T>`, которая содержит два поля типа `T`. Добавьте ограничение на тип `T` для поддержки сложения.

#### Список идентификаторов:
|Имя переменной|Тип данных|Описание и смысл|
|---|---|------|
|`int_pair`|`Pair<i32>`|Пара целых чисел|
|`float_pair`|`Pair<f32>`|Пара чисел с плавающей точкой|
|`sum_int`|`u32`|Сумма целых чисел (преобразованная в u32)|
|`sum_float`|`u32`|Сумма чисел с плавающей точкой (преобразованная в u32)|

#### Код программы:
```Rust
trait Summable {
    fn sum(&self) -> u32;
}

struct Pair<T> {
    first: T,
    second: T,
}

// Реализация для целых чисел
impl Summable for Pair<i32> {
    fn sum(&self) -> u32 {
        (self.first + self.second) as u32
    }
}

// Реализация для чисел с плавающей точкой
impl Summable for Pair<f32> {
    fn sum(&self) -> u32 {
        (self.first + self.second) as u32
    }
}

fn main() {
    let int_pair = Pair { first: 15, second: 25 };
    let sum_int = int_pair.sum();
    println!("Сумма целых чисел: {}", sum_int);

    let float_pair = Pair { first: 10.5, second: 20.3 };
    let sum_float = float_pair.sum();
    println!("Сумма чисел с плавающей точкой (приведенная к u32): {}", sum_float);
}
```
#### Результаты выполнения:
<img width="884" height="96" alt="image" src="https://github.com/user-attachments/assets/fbf9afc8-8baf-4a1f-9acd-cdc65b389fcd" />

## Выполнила: Жукова София Романовна 1об ПОО
