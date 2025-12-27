#include <stdio.h>
#include <string.h>
#include "student.h"

// Функция для заполнения данных студента
void addStudent(struct Student *s, const char *name, int age, float grade) {
    strcpy(s->name, name);
    s->age = age;
    s->grade = grade;
}

// Функция для вывода информации о студенте
void printStudent(const struct Student *s) {
    printf("Имя: %s\n", s->name);
    printf("Возраст: %d\n", s->age);
    printf("Средний балл: %.2f\n", s->grade);
}

// Функция для сохранения массива студентов в файл
int saveStudentsToFile(struct Student students[], int count, const char *filename) {
    FILE *file = fopen(filename, "w");  // "w" - создание/перезапись файла
    if (file == NULL) {
        printf("Ошибка: не удалось создать файл '%s'\n", filename);
        return 0;  // возвращаем 0 при ошибке
    }
    
    printf("Сохранение %d студентов в файл '%s'...\n", count, filename);
    
    // Записываем каждого студента в файл
    for (int i = 0; i < count; i++) {
        fprintf(file, "%s %d %.2f\n", 
                students[i].name, 
                students[i].age, 
                students[i].grade);
    }
    
    fclose(file);
    printf("Данные успешно сохранены!\n");
    return 1;  // возвращаем 1 при успехе
}
