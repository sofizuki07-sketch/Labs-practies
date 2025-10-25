# Лабораторная работа № 5

## Тема: Файловый ввод-вывод

### Задача 1 - Чтение и вывод содержимого текстового файла

#### Постановка задачи
Напишите программу, которая открывает текстовый файл (например, “input.txt”) для чтения, считывает его построчно с помощью функции fgets() и выводит каждую строку на стандартный вывод. Программа должна проверять, успешно ли открыт файл, и корректно закрывать его после чтения. 
#### Список идентификаторов 
| Имя переменной | Тип данных | Описание и смысл | 
|----|----|----------| 
| fp| FILE * | Указатель на файл для чтения | 
| buffer | char | Массив для хранения прочитанных строк | 
#### Код программы

``` c
#include <stdio.h>
#include <stdlib.h>

void readAndPrintFile() {
    FILE *fp = fopen("input.txt", "r");

    if (fp == NULL) {
        perror("Ошибка открытия файла");
        exit(EXIT_FAILURE);
    }

    char buffer[256];
    while (fgets(buffer, sizeof(buffer), fp) != NULL) {
        printf("%s", buffer);
    }

    fclose(fp);
}

int main(void) {
    readAndPrintFile();
    return 0;
}
```

#### Результат работы программы
<img width="584" height="444" alt="6f025cf11622505d8c519acc233ff048" src="https://github.com/user-attachments/assets/c18139e8-5c10-4d0a-8841-933016664f9f" />

### Задача 2 - Запись пользовательского ввода в текстовый файл

#### Постановка задачи
Создайте программу, которая запрашивает у пользователя несколько строк текста, а затем записывает введённые данные в файл “output.txt”. Используйте режим записи “w”. После завершения записи файл закрывается, а программа выводит сообщение об успешном завершении. 
#### Список идентификаторов 
| Имя переменной | Тип данных | Смысл и описание |
|----|----|--------|
| fp | FILE * | Указатель на файл | 
| line | char | Массив-строка, в которой хранится вводимая строка | 
#### Код программы

``` c
#include <stdio.h>
#include <stdlib.h>

void writeUserInputToFile() {

    FILE *fp = fopen("output.txt", "w");

    if (fp == NULL) {
        perror("Ошибка создания файла");
        exit(EXIT_FAILURE);
    }

    char line[256];
    printf("Введите строки текста (для завершения введите пустую строку):\n");
    
    while (1) {
        fgets(line, sizeof(line), stdin);
        if (line[0] == '\n') break;
        fputs(line, fp);
    }
    fclose(fp);
    printf("Данные успешно записаны в output.txt\n");
}

int main(void) {
    writeUserInputToFile();
    return 0;
}
```

#### Результат работы программы
<img width="978" height="308" alt="52e0f1c5d35ff9edb8f6d9f9dab60037" src="https://github.com/user-attachments/assets/33149955-357c-4511-b82e-af72b5edf913" />
<img width="912" height="480" alt="733e3c07d66aeb6e880dd355bba8f2d5" src="https://github.com/user-attachments/assets/fb7e80f2-3ca5-491a-99df-bcd225bdce74" />

### Задача 3 - Копирование содержимого одного файла в другой 
#### Постановка задачи 
Напишите программу, которая копирует содержимое файла “source.txt” в новый файл “destination.txt”. Программа должна открывать
исходный файл в режиме чтения, а целевой — в режиме записи. Содержимое копируется блоками (например, по 256 байт) с использованием функций fread() и fwrite(). 
#### Список идентификаторов 
| Имя переменной | Тип данных | Описание и смысл |
|----------------|------------|------------------|
| src | FILE* | Указатель на исходный файл для чтения |
| dest | FILE* | Указатель на целевой файл для записи |
| buffer | char | Буфер для хранения данных при копировании |
| bytes | size_t | Количество фактически прочитанных байт |
| source | const char* | Имя исходного файла в функции (параметр функции) |
| destination | const char* | Имя целевого файла в функции (параметр функции) |
#### Код программы

``` c
#include <stdio.h>
#include <stdlib.h>

void copyFile(const char *source, const char *destination) {
    FILE *src = fopen(source, "rb");  // Открытие исходного файла для чтения в бинарном режиме
    if (src == NULL) {
        perror("Ошибка открытия исходного файла!");
        exit(EXIT_FAILURE);
    }
    
    FILE *dest = fopen(destination, "wb");  // Создание/открытие целевого файла для записи в бинарном режиме
    if (dest == NULL) {
        perror("Ошибка создания файла назначения!");
        fclose(src);
        exit(EXIT_FAILURE);
    }
    
    char buffer[256];  // Буфер для копирования данных
    size_t bytes;      // Количество прочитанных байт
    
    while ((bytes = fread(buffer, 1, sizeof(buffer), src)) > 0) {
        fwrite(buffer, 1, bytes, dest);  // Запись прочитанных данных в целевой файл
    }
    
    fclose(src);
    fclose(dest);
    printf("Файл успешно скопирован из %s в %s\n", source, destination);
}

int main(void) {
    copyFile("source.txt", "destination.txt");
    return 0;
}
```

#### Результат работы программы
<img width="1676" height="997" alt="image" src="https://github.com/user-attachments/assets/c1b4daae-27bc-4092-ba90-f3c1b9be5236" />

### Задача 4 - Подсчет строк, слов и символов в текстовом файле

#### Постановка задачи
Разработайте программу, которая открывает текстовый файл (например,“input.txt”) и подсчитывает: - количество строк (по числу символов новой строки), - количество слов (слова разделены пробелами и знаками препинания), - количество символов (включая пробелы). После подсчета программа выводит результаты. 
#### Список идентификаторов
| Имя переменной | Тип данных | Описание и смысл |
|----|----|----|
| filename | const char * | Указатель на строку с именем файла для подсчёта |
| fp | FILE * | Указатель на файл для рааботы с ним |
| lines | int | Счётчик количества строк в файле |
| words | int | Счётчик количества слов в файле |
| chars | int | Счётчик количества символов в файле |
| inWord | int | Флаг состояния курсора: 0-вне слова, 1-в слове |
| ch | int | Переменная для хранения текущего символа из файла |

#### Код программы

``` c
#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

// Функция для подсчета статистики файла
void countFileStats(const char *filename) {
    
    FILE *fp = fopen(filename, "r"); // Открываем файл для чтения
    
    if (fp == NULL) {
        perror("Ошибка открытия файла"); // Проверяем успешность открытия файла
        exit(EXIT_FAILURE);  
    }
    
    int lines = 1, words = 0, chars = 0;
    int inWord = 0;  // Флаг: 0 - вне слова, 1 - внутри слова
    char ch; 
    
    // Читаем файл посимвольно до конца файла
    while ((ch = fgetc(fp)) != EOF) {
        chars++;  // Увеличиваем счетчик символов
        
        if (ch == '\n')   // Если символ - перевод строки, увеличиваем счетчик строк
            lines++;
        
        if (isspace(ch)) // Если символ - пробельный (пробел, табуляция, перевод строки)
            inWord = 0;  
        else if (!inWord) {
            inWord = 1;  
            words++;     // Увеличиваем счетчик слов
        }
    }
    
    fclose(fp);
    printf("Строк: %d\nСлов: %d\nСимволов: %d\n", lines, words, chars);
}

int main(void) {
    countFileStats("input4.txt");
    return 0;  
}
```

#### Результат работы программы

<img width="1664" height="456" alt="image" src="https://github.com/user-attachments/assets/a7b1c636-81cb-4986-ac53-2b39755ab117" />
### Задача 5 - Запись и чтение структур в бинарном файле

#### Постановка задачи
Определите структуру (например, struct Student с полями name, age и grade). Создайте массив таких структур, затем запишите его в бинарный файл с помощью fwrite(). После этого откройте файл для чтения и восстановите массив с помощью fread(), после чего выведите данные на экран 
#### Список идентификаторов 
| Имя переменной | Тип данных | Описание и смысл |
|----------------|------------|------------------|
| Student | struct | Структура для данных о студенте |
| name | char[50] | Поле структуры, имя студента |
| age | int | Поле структуры, возраст студента |
| grade | float | Поле структуры, оценка студента |
| filename | const char* | Указатель на строку с именем файла |
| students | struct Student[] | Массив структур с данными о студентах |
| count | int | Количество студентов в массиве |
| fp | FILE* | Указатель на файловый поток |
| studentsToWrite | struct Student[2] | Массив структур для записи в файл |
| studentsRead | struct Student[2] | Массив структур для чтения из файла |
| i | int | Параметр цикла | 
#### Код программы

``` c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct Student{      // структура данных о студенте
    char name[50];
    int age;
    float grade;
};

void writeStudents(const char *filename, struct Student students[], int count){ // функция записи массива структур в бинарный файл
    FILE *fp = fopen(filename, "wb");

    if(fp == NULL){
        perror("Ошибка создания бинарного файла!");
        exit(EXIT_FAILURE);
    }

    fwrite(students, sizeof(struct Student), count, fp);
    fclose(fp);
}

void readStudents(const char *filename, struct Student students[], int count){ // функция создания массива структур из данных файла

    FILE *fp = fopen(filename, "rb");

    if(fp == NULL){
        perror("Ошибка открытия бинарного файла!");
        exit(EXIT_FAILURE);
    }

    fread(students, sizeof(struct Student), count, fp);
    fclose(fp);
}

int main(){
    struct Student studentsToWrite[2] = {  // создаём массив структур, который будем записывать в файл
        {"Иванов", 20, 4.5f},
        {"Петров", 22, 3.8f}
    };
    writeStudents("students.bin", studentsToWrite, 2);

    struct Student studentsRead[2]; // создаём массив структур, в который будем записывать с файла
    readStudents("students.bin", studentsRead, 2);

    for (int i = 0; i < 2; i++) { // выводим красиво содержание массива с файла
        printf("Студент: %s, Возраст: %d, Оценка: %.2f\n",
        studentsRead[i].name, studentsRead[i].age, studentsRead[i].grade);
    }
    
    return 0;
}
```

#### Результат работы программы
<img width="1568" height="531" alt="image" src="https://github.com/user-attachments/assets/bedc8b7b-1a10-4619-8c5d-885997c8522b" />

### Выполнила: Жукова София 1об ПОО
