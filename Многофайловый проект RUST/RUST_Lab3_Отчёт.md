# Лабораторная работа №3 RUST
## Тема: Комплексная разработка консольного приложения на Rust
## Проект: Система управления студенческой библиотекой
- - - - - - - 
### Тестирование программы:
#### **Первый тест:**
запуск и добавление книг
<img width="1516" height="1310" alt="2025-12-26_23-26-02" src="https://github.com/user-attachments/assets/e4a3fdee-b707-4454-b334-52cbd04289e6" />
добавление читателей
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
**models.rs** (тип данных у year - String, т.к. нам нет необходимости взаимодействовать с ним как с числом, и памяти займёт столько же)
добаавление нового поля в структуру книги
<img width="976" height="759" alt="2025-12-27_14-54-26" src="https://github.com/user-attachments/assets/ac0f2f97-6902-43b6-8239-de154d4c9a31" />
**lib.rs**
добавление нового поля при создании переменных структур
<img width="1289" height="885" alt="2025-12-27_14-59-56" src="https://github.com/user-attachments/assets/b495fd37-8416-4715-82b6-f97f08ea85c1" />
**main.rs**
функция вывода книг изменена - добавлен год
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
**lib.rs**
публичная функция поиска книги
<img width="1419" height="414" alt="2025-12-27_16-22-40" src="https://github.com/user-attachments/assets/3199a2f8-8675-4dcb-a8aa-87d83eea7623" />
**main.rs**
match по выбору из меню
<img width="1434" height="738" alt="2025-12-27_16-15-23" src="https://github.com/user-attachments/assets/47ec63b4-dafb-4b58-9161-7da8f1e837c1" />
меню с новым пунктом 7
<img width="1311" height="737" alt="2025-12-27_16-17-15" src="https://github.com/user-attachments/assets/48e46b4d-7627-42bd-8ee1-d7e169dc8990" />
функция поиска книги
<img width="1698" height="1037" alt="2025-12-27_16-02-04" src="https://github.com/user-attachments/assets/4ccc9fca-c460-485f-81ca-833865a318a8" />
<img width="1650" height="956" alt="2025-12-27_16-02-27" src="https://github.com/user-attachments/assets/20c9cc33-8d50-44aa-9685-c7d5c15cb9a8" />
#### Тестирование работы программы:
запуск и вывод обновлённого меню
<img width="995" height="574" alt="2025-12-27_15-59-28" src="https://github.com/user-attachments/assets/f45c2924-673f-4183-9954-0dc7d53fe71c" />
поиск сохранённой книги
<img width="1513" height="528" alt="2025-12-27_16-00-20" src="https://github.com/user-attachments/assets/a8bf6480-cc98-49bc-b47e-0067833d4c9f" />
поиск несохранённой книги
<img width="1043" height="247" alt="2025-12-27_16-00-49" src="https://github.com/user-attachments/assets/49b1a6f9-69bd-4235-944b-56753ae57656" />
выход из программы под новым номером
<img width="1465" height="350" alt="2025-12-27_16-01-24" src="https://github.com/user-attachments/assets/5fc9130f-2c2b-40b5-b871-db1e4b150adb" />

### Задание 3
#### Постановка задачи:
Добавьте отслеживание того, какие книги выданы каждому читателю. Создайте пункт меню “Показать книги читателя”.
#### Изменённый код:
**models.rs**
добавление поля с вектором книг в структуру читателя
<img width="1347" height="295" alt="2025-12-27_17-07-14" src="https://github.com/user-attachments/assets/b817d429-e66f-4882-b844-36bc418d91a8" />
**lib.rs**
регистрация читателя с пустым вектором книг
<img width="1326" height="529" alt="2025-12-27_18-16-29" src="https://github.com/user-attachments/assets/e1f44165-eb50-41df-b18d-5bd01d49e6cb" />
выдача книги с добавлением её к читателю
<img width="1692" height="1067" alt="image" src="https://github.com/user-attachments/assets/12b1e398-dbf1-4913-b534-744d7c602e9c" />
возврат книги с удалением её у читателя
<img width="1622" height="982" alt="image" src="https://github.com/user-attachments/assets/233e0367-1330-4139-a692-3cae257aaa4b" />
функция получения книг читателя
<img width="1570" height="577" alt="image" src="https://github.com/user-attachments/assets/58f9413b-f2dd-44af-b923-d3aebd487d26" />
**main.rs**
Обновление меню
<img width="1491" height="764" alt="2025-12-27_17-19-31" src="https://github.com/user-attachments/assets/460d0dd4-6835-4d42-8fe2-43b513fb4a0e" />
<img width="1520" height="770" alt="2025-12-27_17-18-30" src="https://github.com/user-attachments/assets/59b3d994-d13c-46d9-8c21-7783afe942c5" />
функция вывода книг читателя
<img width="2140" height="947" alt="2025-12-27_17-25-02" src="https://github.com/user-attachments/assets/e1b4a222-616d-4b81-b207-49ab5626d61a" />
<img width="2117" height="986" alt="2025-12-27_17-25-25" src="https://github.com/user-attachments/assets/08e22e66-3e48-4a44-947a-41af24d10d99" />

#### Тестирование работы программы:
запуск и обновлённое меню
<img width="2155" height="1214" alt="2025-12-27_17-49-59" src="https://github.com/user-attachments/assets/0ef71019-0580-47f9-8c01-e4471265c8f5" />
выдача книг
<img width="1428" height="992" alt="2025-12-27_18-02-40" src="https://github.com/user-attachments/assets/a7a2127f-738d-4544-87f0-bb60042e0ad9" />
список книг читателя
<img width="1370" height="519" alt="2025-12-27_18-05-59" src="https://github.com/user-attachments/assets/d1d405a6-23bf-49e6-ae0a-e4b954ee00a2" />
возврат и список книг читателя
<img width="1401" height="923" alt="2025-12-27_18-09-05" src="https://github.com/user-attachments/assets/51fb7ade-34ff-4ae8-9080-985967e76983" />

## Выполнила: Жукова София 1об ПОО




