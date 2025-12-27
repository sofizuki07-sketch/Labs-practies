use std::io::{self, Write};
use library_manager::Library;

// Константа с именем файла для хранения данных
const DB_FILE: &str = "library.json";

fn main() {
    // Пытаемся загрузить библиотеку из файла
    // Если файл не существует или поврежден, создаем новую библиотеку
    let mut library = Library::load_from_file(DB_FILE).unwrap_or_else(|_| {
        println!("Файл данных не найден. Создана новая библиотека.");
        Library::new()
        });
    println!("=== Система управления студенческой библиотекой ===");
    println!("Добро пожаловать!\n");

    // Основной цикл приложения
    loop {
        print_menu();
        let choice = read_line().trim().to_string();
        match choice.as_str() {
            "1" => add_book(&mut library),
            "2" => register_reader(&mut library),
            "3" => borrow_book(&mut library),
            "4" => return_book(&mut library),
            "5" => list_books(&library),
            "6" => list_readers(&library),
            "7" => search_books(&library),
            "8" => show_reader_books(&library),
            "9" => {
            println!("\nЗавершение работы...");
            break;
            }
            _ => println!("\n Неверный выбор. Пожалуйста, выберите пункт от 1 до 9."),
        }
    }

    // Демонстрация unsafe (опционально)
    show_library_memory_address(&library);
    // Сохранение данных перед выходом
    match library.save_to_file(DB_FILE) {
        Ok(_) => println!(" Данные успешно сохранены в '{}'.", DB_FILE),
        Err(e) => eprintln!(" Ошибка при сохранении данных: {}", e),
    }
    println!("До свидания!");
}


/// Выводит главное меню
fn print_menu() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│            ГЛАВНОЕ МЕНЮ             │");
    println!("├─────────────────────────────────────┤");
    println!("│ 1. Добавить книгу                   │");
    println!("│ 2. Зарегистрировать читателя        │");
    println!("│ 3. Выдать книгу                     │");
    println!("│ 4. Вернуть книгу                    │");
    println!("│ 5. Показать список книг             │");
    println!("│ 6. Показать список читателей        │");
    println!("│ 7. Найти книгу по названию          │");
    println!("│ 8. Показать книги читателя          │");
    println!("│ 9. Выход                            │");
    println!("└─────────────────────────────────────┘");
    print!("\nВаш выбор: ");
    // flush() гарантирует, что приглашение отобразится до ввода
    io::stdout().flush().unwrap();
}

/// Читает строку из стандартного ввода
fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Не удалось прочитать строку");
    input
}

/// Функция добавления книги
fn add_book(library: &mut Library) {
    println!("\n--- Добавление книги ---");
    print!("Введите название книги: ");
    io::stdout().flush().unwrap();
    let title = read_line().trim().to_string();
    print!("Введите автора книги: ");
    io::stdout().flush().unwrap();
    let author = read_line().trim().to_string();
    print!("Введите год издания книги: ");
    io::stdout().flush().unwrap();
    let year = read_line().trim().to_string();

    // Валидация ввода
    if title.is_empty() || author.is_empty() || year.is_empty() {
        println!("Ошибка: название, автор и год издания не могут быть пустыми.");
        return;
    }
    let book = library.add_book(title, author, year);
    println!("Книга '{}' (автор: {} год: {}) успешно добавлена с ID {}.",
    book.title, book.author, book.year, book.id);
}

/// Функция регистрации читателя
fn register_reader(library: &mut Library) {
    println!("\n--- Регистрация читателя ---");
    print!("Введите имя читателя: ");
    io::stdout().flush().unwrap();
    let name = read_line().trim().to_string();
    if name.is_empty() {
    println!("Ошибка: имя не может быть пустым.");
    return;
    }
    let reader = library.register_reader(name);
    println!("Читатель '{}' зарегистрирован с ID {}.", reader.name, reader.id);
}

/// Функция выдачи книги
fn borrow_book(library: &mut Library) {
    println!("\n--- Выдача книги ---");
    print!("Введите ID книги: ");
    io::stdout().flush().unwrap();
    let book_id: u32 = match read_line().trim().parse() {
    Ok(id) => id,
    Err(_) => {
        println!("Ошибка: некорректный ID книги.");
        return;
    }
    };
    print!("Введите ID читателя: ");
    io::stdout().flush().unwrap();
    let reader_id: u32 = match read_line().trim().parse() {
    Ok(id) => id,
    Err(_) => {
        println!("Ошибка: некорректный ID читателя.");
        return;
    }
    };
    // Вызываем метод и обрабатываем результат
    match library.borrow_book(book_id, reader_id) {
        Ok(_) => println!("Книга успешно выдана."),
        Err(e) => println!("Ошибка: {}", e),
    }
}

/// Функция возврата книги
fn return_book(library: &mut Library) {
    println!("\n--- Возврат книги ---");
    print!("Введите ID книги: ");
    io::stdout().flush().unwrap();
    let book_id: u32 = match read_line().trim().parse() {
    Ok(id) => id,
    Err(_) => {
        println!("Ошибка: некорректный ID книги.");
        return;
    }
    };
    match library.return_book(book_id) {
        Ok(_) => println!("Книга успешно возвращена."),
        Err(e) => println!("Ошибка: {}", e),
    }
}

/// Функция вывода списка книг
fn list_books(library: &Library) {
    let books = library.list_books();
    if books.is_empty() {
        println!("\nВ библиотеке пока нет книг.");
        return;
    }
    println!("\n┌────────────────────────────────────────────────────────────────────────────┐");
    println!("│                               СПИСОК КНИГ                                  │");
    println!("├────┬──────────────────────────┬──────────────────────┬──────┬──────────────┤");
    println!("│ ID │ Название                 │ Автор                │ Год  │ Статус       │");
    println!("├────┼──────────────────────────┼──────────────────────┼──────┼──────────────┤");
    for book in books {
        let status = if book.is_available {
            "Доступна"
        } else {
            "Выдана"
        };
        println!("│ {:2} │ {:<24} │ {:<20} │ {:4} │ {:<12} │",
        book.id,
        truncate(&book.title, 24),
        truncate(&book.author, 20),
        truncate(&book.year, 4),
        status);
    }
    println!("└────┴──────────────────────────┴──────────────────────┴──────┴──────────────┘");
}

/// Функция вывода списка читателей
fn list_readers(library: &Library) {
    let readers = library.list_readers();
    if readers.is_empty() {
        println!("\nНет зарегистрированных читателей.");
        return;
    }
    println!("\n┌──────────────────────────────────────────────┐");
    println!("│               СПИСОК ЧИТАТЕЛЕЙ               │");
    println!("├────┬─────────────────────────────────────────┤");
    println!("│ ID │ Имя                                     │");
    println!("├────┼─────────────────────────────────────────┤");
    for reader in readers {
        println!("│ {:2} │ {:<39} │", reader.id, truncate(&reader.name, 39));
    }
    println!("└────┴─────────────────────────────────────────┘");
}

/// Вспомогательная функция для обрезки строк
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len-3])
    }
}

/// Функция поиска книг по названию
fn search_books(library: &Library) {
    println!("\n--- Поиск книги по названию ---");
    print!("Введите часть названия для поиска: ");
    io::stdout().flush().unwrap();
    
    let query = read_line().trim().to_string();
    
    if query.is_empty() {
        println!("Ошибка: поисковый запрос не может быть пустым.");
        return;
    }
    
    let found_books = library.search_books_by_title(&query);
    
    if found_books.is_empty() {
        println!("Книги по запросу '{}' не найдены.", query);
        return;
    }
    
    println!("\n┌────────────────────────────────────────────────────────────────────────────┐");
    println!("│                         РЕЗУЛЬТАТЫ ПОИСКА                                  │");
    println!("├────┬──────────────────────────┬──────────────────────┬──────┬──────────────┤");
    println!("│ ID │ Название                 │ Автор                │ Год  │ Статус       │");
    println!("├────┼──────────────────────────┼──────────────────────┼──────┼──────────────┤");
    
    for book in found_books.iter() {
        let status = if book.is_available {
            "Доступна"
        } else {
            "Выдана"
        };
        println!("│ {:2} │ {:<24} │ {:<20} │ {:4} │ {:<12} │",
        book.id,
        truncate(&book.title, 24),
        truncate(&book.author, 20),
        truncate(&book.year, 4),
        status);
    }
    println!("└────┴──────────────────────────┴──────────────────────┴──────┴──────────────┘");
    println!("Найдено книг: {}", found_books.len());
}

/// Функция показа книг читателя
fn show_reader_books(library: &Library) {
    println!("\n--- Книги читателя ---");
    print!("Введите ID читателя: ");
    io::stdout().flush().unwrap();
    
    let reader_id: u32 = match read_line().trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Ошибка: некорректный ID читателя.");
            return;
        }
    };
    
    match library.get_reader_books(reader_id) {
        Ok(books) => {
            if books.is_empty() {
                // Получаем имя читателя для лучшего сообщения
                if let Some(reader) = library.list_readers().iter().find(|r| r.id == reader_id) {
                    println!("У читателя '{}' (ID: {}) нет выданных книг.", reader.name, reader_id);
                } else {
                    println!("У читателя с ID {} нет выданных книг.", reader_id);
                }
                return;
            }
            
            // Получаем имя читателя
            let reader_name = if let Some(reader) = library.list_readers().iter().find(|r| r.id == reader_id) {
                &reader.name
            } else {
                "Неизвестный читатель"
            };
            
            println!("\n┌────────────────────────────────────────────────────────────────────────────┐");
            println!("│                   КНИГИ ЧИТАТЕЛЯ: {:<40} │", truncate(reader_name, 40));
            println!("├────┬──────────────────────────┬──────────────────────┬──────┬──────────────┤");
            println!("│ ID │ Название                 │ Автор                │ Год  │ Статус       │");
            println!("├────┼──────────────────────────┼──────────────────────┼──────┼──────────────┤");
            
            for book in &books {
                let status = if book.is_available {
                    "Доступна"
                } else {
                    "Выдана"
                };
                println!("│ {:2} │ {:<24} │ {:<20} │ {:4} │ {:<12} │",
                book.id,
                truncate(&book.title, 24),
                truncate(&book.author, 20),
                truncate(&book.year, 4),
                status);
            }
            println!("└────┴──────────────────────────┴──────────────────────┴──────┴──────────────┘");
            println!("Всего книг: {}", books.len());
        }
        Err(e) => println!("Ошибка: {}", e),
    }
}

/// Демонстрация работы с unsafe (опционально)
fn show_library_memory_address(library: &Library) {
    // Создаем сырой указатель на объект Library
    let raw_ptr: *const Library = library;
    // Работа с сырыми указателями требует unsafe блока
    unsafe {
        println!("\n[Unsafe-демонстрация]");
        println!(" Объект Library находится в памяти по адресу: {:p}", raw_ptr);
    }
}
