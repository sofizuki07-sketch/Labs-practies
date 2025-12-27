# Лабораторная работа №3 RUST
## Тема: Комплексная разработка консольного приложения на Rust
## Проект: Система управления студенческой библиотекой
- - - - - - - 
### Тестирование программы:
#### **Первый тест:**
запуск и добавление книг
<img width="1516" height="1310" alt="2025-12-26_23-26-02" src="https://github.com/user-attachments/assets/e4a3fdee-b707-4454-b334-52cbd04289e6" />
добавление читателеей
<img width="1378" height="869" alt="2025-12-26_23-27-58" src="https://github.com/user-attachments/assets/c8244752-b7e6-45b0-92e0-ed6743e89902" />
список книг и список читателей
<img width="1685" height="455" alt="2025-12-26_23-39-45" src="https://github.com/user-attachments/assets/264b6f03-9808-492c-9cbc-3124831e41b3" />
<img width="1573" height="453" alt="2025-12-26_23-40-31" src="https://github.com/user-attachments/assets/76229480-7b74-4b81-b830-79579381a052" />
выдача книги и проверка её статуса в списке
<img width="1565" height="1068" alt="2025-12-27_00-06-44" src="https://github.com/user-attachments/assets/e972ffa0-cba8-4e34-9ffe-768d02de2861" />
попытка выдачи недоступной книги и завершение работы
<img width="1608" height="1020" alt="2025-12-27_00-07-41" src="https://github.com/user-attachments/assets/3ff50d17-9e49-4796-88b6-6cdc093ac2c0" />
2 запуск проекта (проверка на отсутствие создания новой библиотеки -> library.json есть)
<img width="1616" height="861" alt="2025-12-27_00-11-48" src="https://github.com/user-attachments/assets/82942f2c-b462-4391-afc5-cc3fce02bebe" />
список книг и статусов сохранён
<img width="1617" height="1410" alt="2025-12-27_00-11-35" src="https://github.com/user-attachments/assets/e776d4aa-a1f0-4b97-8b30-a40c55538b31" />

### Задание 1
#### Постановка задачи:
Добавьте поле year: u32 (год издания) в структуру Book и отобразите его в списке книг.
#### Изменённый код:
models.rs
<img width="976" height="759" alt="2025-12-27_14-54-26" src="https://github.com/user-attachments/assets/ac0f2f97-6902-43b6-8239-de154d4c9a31" />
lib.rs
<img width="1289" height="885" alt="2025-12-27_14-59-56" src="https://github.com/user-attachments/assets/b495fd37-8416-4715-82b6-f97f08ea85c1" />
main.rs
<img width="1265" height="1043" alt="2025-12-27_15-06-57" src="https://github.com/user-attachments/assets/aa1cb7ce-6485-4c7d-aadc-4cb5a6e2ad21" />
<img width="1864" height="1031" alt="2025-12-27_15-08-48" src="https://github.com/user-attachments/assets/c162d690-7532-47bf-b6ea-6faccaf9e774" />
#### Тестирование работы программы:
добавление книги
<img width="1237" height="291" alt="2025-12-27_15-10-29" src="https://github.com/user-attachments/assets/abe28fb0-6c8c-4927-8cb5-7cef4f6e688b" />
список книг
<img width="1322" height="394" alt="2025-12-27_15-10-52" src="https://github.com/user-attachments/assets/2916fc37-8919-43d9-bea9-20269f7e4096" />

### Задание 2
#### Постановка задачи:
Реализуйте функцию поиска книг по названию (частичное совпадение, без учета регистра).
#### Изменённый код:

#### Тестирование работы программы:


### Задание 3
#### Постановка задачи:
#### Изменённый код:
#### Тестирование работы программы:

### Задание 4
#### Постановка задачи:
#### Изменённый код:
#### Тестирование работы программы:

### Задание 5
#### Постановка задачи:
#### Изменённый код:
#### Тестирование работы программы:



