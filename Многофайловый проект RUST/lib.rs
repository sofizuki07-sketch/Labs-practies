use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

// Объявляем модуль models и делаем его публичным
pub mod models;
use models::{Book, Reader};

/// Перечисление возможных ошибок библиотеки
#[derive(Debug)]
pub enum LibraryError {
    BookNotFound,
    BookNotAvailable,
    ReaderNotFound,
    InvalidInput,
}
/// Реализация трейта Display для красивого вывода ошибок
impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LibraryError::BookNotFound => write!(f, "Книга не найдена."),
            LibraryError::BookNotAvailable => write!(f, "Книга уже выдана."),
            LibraryError::ReaderNotFound => write!(f, "Читатель не найден."),
            LibraryError::InvalidInput => write!(f, "Некорректный ввод."),
        }
    }
}


/// Основная структура библиотеки
#[derive(Serialize, Deserialize)]
pub struct Library {
    books: Vec<Book>, // Вектор всех книг
    readers: HashMap<u32, Reader>, // Хеш-таблица читателей (ключ = ID)
    next_book_id: u32, // Счетчик для генерации ID книг
    next_reader_id: u32, // Счетчик для генерации ID читателей
}

impl Library {
    /// Создает новую пустую библиотеку
    pub fn new() -> Self {
        Self {
            books: Vec::new(),
            readers: HashMap::new(),
            next_book_id: 1,
            next_reader_id: 1,
        }
    }

    /// Добавляет новую книгу в библиотеку
    /// Возвращает ссылку на добавленную книгу
    pub fn add_book(&mut self, title: String, author: String, year: String) -> &Book {
        let new_book = Book {
            id: self.next_book_id,
            title,
            author,
            year,
            is_available: true,
        };
        self.books.push(new_book);
        self.next_book_id += 1;
        // unwrap() здесь безопасен, так как мы только что добавили элемент
        self.books.last().unwrap()
    }

    /// Регистрирует нового читателя
    /// Возвращает ссылку на зарегистрированного читателя
    pub fn register_reader(&mut self, name: String) -> &Reader {
        let new_reader = Reader {
            id: self.next_reader_id,
            name,
            borrowed_books: Vec::new(),
        };
        self.readers.insert(new_reader.id, new_reader);
        self.next_reader_id += 1;
        // unwrap() безопасен, так как мы только что вставили элемент
        self.readers.get(&(self.next_reader_id - 1)).unwrap()
    }

    /// Находит книгу по ID и возвращает изменяемую ссылку
    /// Возвращает Option, так как книга может не существовать
    pub fn find_book_by_id(&mut self, id: u32) -> Option<&mut Book> {
        self.books.iter_mut().find(|b| b.id == id)
    }

    /// Выдает книгу читателю
    /// Возвращает Result: Ok(()) при успехе, Err(LibraryError) при ошибке
       pub fn borrow_book(&mut self, book_id: u32, reader_id: u32) -> Result<(), LibraryError> {
        // Проверяем существование читателя
        if !self.readers.contains_key(&reader_id) {
            return Err(LibraryError::ReaderNotFound);
        }
        
        // Находим индекс книги в векторе
        let book_index = self.books.iter()
            .position(|b| b.id == book_id)
            .ok_or(LibraryError::BookNotFound)?;
            
        // Проверяем доступность книги
        if !self.books[book_index].is_available {
            return Err(LibraryError::BookNotAvailable);
        }
        
        // Обновляем книгу
        self.books[book_index].is_available = false;
        
        // Обновляем читателя
        if let Some(reader) = self.readers.get_mut(&reader_id) {
            reader.borrowed_books.push(book_id);
        }
        
        Ok(())
    }
    
    /// Возвращаает книгу в библиотеку
    pub fn return_book(&mut self, book_id: u32) -> Result<(), LibraryError> {
        // Находим индекс книги в векторе
        let book_index = self.books.iter()
            .position(|b| b.id == book_id)
            .ok_or(LibraryError::BookNotFound)?;
            
        // Проверяем, выдана ли книга
        if self.books[book_index].is_available {
            return Ok(());
        }
        
        // Ищем и обновляем читателя
        for reader in self.readers.values_mut() {
            if let Some(pos) = reader.borrowed_books.iter().position(|&id| id == book_id) {
                reader.borrowed_books.remove(pos);
                break;
            }
        }
        
        // Обновляем книгу
        self.books[book_index].is_available = true;
        
        Ok(())
    }

    /// Возвращает ссылку на список всех книг
    pub fn list_books(&self) -> &Vec<Book> {
        &self.books
    }

    /// Возвращает вектор ссылок на всех читателей
    pub fn list_readers(&self) -> Vec<&Reader> {
        self.readers.values().collect()
    }

    /// Поиск книг по названию (частичное совпадение, без учета регистра)
    /// Возвращает вектор книг, содержащих искомую подстроку в названии
    pub fn search_books_by_title(&self, query: &str) -> Vec<&Book> {
        let query_lower = query.to_lowercase();
        self.books
            .iter()
            .filter(|book| book.title.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Получает книги, выданные конкретному читателю
    /// Возвращает вектор книг читателя
    pub fn get_reader_books(&self, reader_id: u32) -> Result<Vec<&Book>, LibraryError> {
        let reader = self.readers.get(&reader_id).ok_or(LibraryError::ReaderNotFound)?;
        
        let mut reader_books = Vec::new();
        for &book_id in &reader.borrowed_books {
            if let Some(book) = self.books.iter().find(|b| b.id == book_id) {
                reader_books.push(book);
            }
        }
        
        Ok(reader_books)
    }

    /// Сохраняет состояние библиотеки в JSON-файл
    pub fn save_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        // Сериализуем структуру в JSON с отступами (pretty)
        let data = serde_json::to_string_pretty(self)?;
        // Создаем файл (или перезаписываем существующий)
        let mut file = File::create(path)?;
        // Записываем данные как байты
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    /// Загружает состояние библиотеки из JSON-файла
    pub fn load_from_file(path: &str) -> Result<Self, std::io::Error> {
        // Открываем файл для чтения
        let mut file = File::open(path)?;
        let mut data = String::new();
        // Читаем весь файл в строку
        file.read_to_string(&mut data)?;
        // Десериализуем JSON в структуру Library
        let library = serde_json::from_str(&data)?;
        Ok(library)
    }
}
