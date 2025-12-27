use serde::{Serialize, Deserialize};

/// Структура, представляющая книгу в библиотеке
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub year: String,
    pub is_available: bool,
}

/// Структура, представляющая читателя
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reader {
    pub id: u32,
    pub name: String,
    pub borrowed_books: Vec<u32>,
}