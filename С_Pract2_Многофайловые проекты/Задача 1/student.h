#ifndef STUDENT_H
#define STUDENT_H

// Определение структуры Student
struct Student {
    char name[50]; // Имя студента
    int age; // Возраст студента
    float grade; // Средний балл
};

// Прототипы функций для работы со студентом
void addStudent(struct Student *s, const char *name, int age, float grade);
void printStudent(const struct Student *s);
int saveStudentsToFile(struct Student students[], int count, const char *filename);

#endif // STUDENT_H