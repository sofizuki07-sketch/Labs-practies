#include <stdio.h>
#include <string.h>
#include "student.h"

int main(void) {
    // Создание массива студентов
    struct Student students[3];

    // Заполнение данных студентов
    addStudent(&students[0], "Иванов", 19, 4.2);
    addStudent(&students[1], "Петров", 21, 4.8);
    addStudent(&students[2], "Сидоров", 20, 3.3);

    // Вывод информации о студентах
    for (int i = 0; i < 3; i++) {
        printf("Студент %d:\n", i + 1);
        printStudent(&students[i]);
        printf("\n");
    }

    // Сохранение студентов в файл
    printf("\n--- Сохранение в файл ---\n");
    saveStudentsToFile(students, 3, "students.txt");
    
    // Показываем содержимое файла
    printf("\n--- Содержимое файла students.txt ---\n");
    FILE *file = fopen("students.txt", "r");
    if (file != NULL) {
        char line[100];
        while (fgets(line, sizeof(line), file)) {
            printf("%s", line);
        }
        fclose(file);
    }

    return 0;
}
