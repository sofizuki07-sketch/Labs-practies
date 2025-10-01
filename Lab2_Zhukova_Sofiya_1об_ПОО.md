# Лабораторная работа № 2

## Тема: Структуры

### Задача 1 - создать структуру с указателем на функцию

#### Постановка задачи

Создайте структуру, одно из полей которой является указателем на функцию. Вызовите эту функцию через имя переменной структуры и поле указателя на функцию.

#### Список идентификаторов
| Имя переменной | Тип данных | Описание и смысл |
|----|----|----|
| x | int | Вводимое с клавиатуры число |
| y | int | Вводимое с клавиатуры число |
| result | int | Результирующая переменная |
| Calculator | struct | Структура, хранящая поле для указателя на функцию |
| culc | struct (Calculator) | Имя переменной структуры, для присвоения значения |
| add | int | Функция сложения 2х целых чисел |

#### Код программы

``` c
#include <stdio.h>

int add(int a, int b){ // функция сложения 2х целых чисел
    return a+b;
}

struct Calculator{ // структура 
    int (*add_function)(int, int);
} culc; // сразу вводим имя переменной структуры

int main() {
   
    int x, y;
    printf("Введите 2 целых числа: \n");
    scanf("%d %d", &x, &y);

    culc.add_function = add; // копируем адрес функции в указатель на структуру

    int result = culc.add_function(x, y); // вызываем функцию через структуру
    printf("Результат сложения: %d", result);

    return 0;
}
```

#### Результат работы программы

<img width="1006" height="348" alt="9e71ed6d40e822e36332f52c04ecd147" src="https://github.com/user-attachments/assets/3bef00e8-a528-4dc2-a01b-cf79e9afeb12" />

### Задача 2 - структура для вектора в трёхмерном пространстве

#### Постановка задачи

Реализуйте структуру для вектора в 3D пространстве и добавьте следующие
функции: 
• Скалярное умножение векторов; • Векторное произведение; 
• Модуль вектора; 
• Распечатка вектора. 
В структуре также должно быть поле для хранения имени вектора.

#### Список идентификаторов

| Имя переменной | Тип данных | Описание и смысл |
|----|----|----|
| x | double | Поле в структуре, координата вектора по оси x |
| y | double | Поле в структуре, координата вектора по оси y |
| z | double | Поле в структуре, координата вектора по оси z |
| name | char\[\] | Поле в структуре, имя вектора |
| Vector3D | struct | Структура для задания вектора 3D |
| result | struct Vector3D | Результирующая переменная(структуры) для значения перемножения векторов |
| v1 | struct Vector3D | Переменная структуры, 1 вектор |
| v2 | struct Vector3D | Переменная структуры, вектор 2 |
| v | struct Vector3D | Переменная структуры, отправляется в часть функций как отдельный рассматриваемый вектор |
| dot | double | Результирующая переменная для значения скалярного произведения векторов |
| mod1 | double | Результирующая переменная для значения модуля вектора 1 |
| mod2 | double | Результирующая переменная для значения модуля вектора 2 |
| cross | struct Vector3D | Результирующая переменная(структуры) для значения произведения векторов |

#### Код программы

``` c
#include <stdio.h>
#include <math.h>
#include <string.h>

struct Vector3D{ // структура 
    char name[50];
    double x;
    double y;
    double z;
} ;

double dot_product(struct Vector3D v1, struct Vector3D v2) { // скалярное произведение
    return v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
}

struct Vector3D cross_product(struct Vector3D v1, struct Vector3D v2) { // векторное произведение
    struct Vector3D result;
    strcpy(result.name, "Векторное произведение");
    result.x = v1.y * v2.z - v1.z * v2.y;
    result.y = v1.z * v2.x - v1.x * v2.z;
    result.z = v1.x * v2.y - v1.y * v2.x;
    return result;
}

double module(struct Vector3D v) { // модуль вектора
    return sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
}

void print_vector(struct Vector3D v) { // вывод вектора
    printf("%s = (%.2f, %.2f, %.2f)\n", v.name, v.x, v.y, v.z);
}

int main() {
 
    struct Vector3D v1 = {"Vector_A", 1.0, 2.0, 3.0}; //создаём переменнные для векторов
    struct Vector3D v2 = {"Vector_B", 4.0, 5.0, 6.0};

    printf("Векторы: \n");
    print_vector(v1);
    print_vector(v2);
    printf("\n");
    
    double dot = dot_product(v1, v2);
    printf("Скалярное произведение = %.2f\n", dot);
    
    struct Vector3D cross = cross_product(v1, v2);
    print_vector(cross);

    double mod1 = module(v1);
    double mod2 = module(v2);
    printf("Модуль %s = %.2f\n", v1.name, mod1);
    printf("Модуль %s = %.2f\n", v2.name, mod2);

    
    return 0;
}
```

#### Результат работы программы

<img width="1042" height="342" alt="739219f8d6ac3e5d6dc441f101f9e1a0" src="https://github.com/user-attachments/assets/da4dbae9-3a68-49cf-aafe-49f438a73e54" />

### Задача 3 - вычисление комплексной экспоненты

#### Постановка задачи

Используя структуру для представления комплексного числа, вычислите комплексную экспоненту 𝑒 ^𝑧 для числа 𝑧 ∈ ℂ. 
Формула экспоненты:
<img width="982" height="200" alt="image" src="https://github.com/user-attachments/assets/b7fb9b19-9b2b-402a-afbc-497442f5be8d" />

**Пояснение** Комплексные числа имеют форму 𝑧 = 𝑎+𝑏𝑖, где 𝑎 — действительная часть, 𝑏 — мнимая часть, а 𝑖 — это мнимая единица, удовлетворяющая уравнению 𝑖^2 = −1. 
Основные арифметические операции над комплексными числами включают: 
1. Сложение двух комплексных чисел:
Если есть два комплексных числа 𝑧1 = 𝑎1 + 𝑏1𝑖 и 𝑧2 = 𝑎2 + 𝑏2𝑖, то их сумма вычисляется по правилу: 𝑧1 + 𝑧2 = (𝑎1 + 𝑎2) + (𝑏1 + 𝑏2)𝑖
Действительные и мнимые части складываются отдельно.
2. Умножение двух комплексных чисел:
Если есть два комплексных числа 𝑧1 = 𝑎1 + 𝑏1𝑖 и 𝑧2 = 𝑎2 + 𝑏2𝑖, то их произведение вычисляется по формуле: 𝑧1 × 𝑧2 = (𝑎1𝑎2 − 𝑏1𝑏2) + (𝑎1𝑏2 + 𝑎2𝑏1)𝑖 Здесь используется дистрибутивное свойство умножения и тот факт, что 𝑖^2 = −1.
3. Возведение в целую степень через умножение:
Чтобы возвести комплексное число 𝑧 = 𝑎+𝑏𝑖 в целую степень 𝑛, можно последовательно умножать число само на себя 𝑛 раз: 𝑧^𝑛 = 𝑧 × 𝑧 × ⋯ × 𝑧(n раз)
 4. Умножение на скаляр:
Чтобы умножить комплексное число 𝑧 = 𝑎 + 𝑏𝑖 на вещественное число 𝜆, нужно умножить как действительную, так и мнимую часть на это число: 𝜆 × 𝑧 = 𝜆𝑎 + 𝜆𝑏𝑖
5. Модуль комплексного числа:
Модуль комплексного числа 𝑧 = 𝑎 + 𝑏𝑖 вычисляется по формуле:
\|𝑧\| = √(𝑎^2 + 𝑏^2)
Модуль — это длина вектора, представляющего комплексное число на комплексной плоскости.

В языке C есть встроенная поддержка работы с комплексными числами через complex.h, однако в данной задаче требуется создать свою структуру и функции для работы с комплексными числами.

#### Список идентификаторов

| Имя переменной | Тип данных | Описание и смысл |
|----|----|----|
| Complex | struct | Структурадля представления комплексного числа |
| real | double | Поле структуры, действительная часть комплексного числа |
| imag | double | Поле структуры, мнимая часть комплексного числа |
| a | struct (Complex) | Аргумент функций(сложение, умножение 2х комп чисел, умножение комп чисоа на скаляр), комплексное число, с которым функция работает |
| b | struct (Complex) | Аргумент функций(сложение, умножение 2х комп чисел), с которым функция работает |
| result | struct (Complex) | Результат работы функций(сложение, умножение, умножение на скаляр),который возвращается также в виде комплексного числа |
| result | long double | результат вычисления факториала |
| scalar | double | вещественное число, на которое умножается комплексное число |
| x | int | число, факториал которого вычисляется |
| z | struct (Complex) | выводимое в функции комплексное число |
| sum | struct (Complex) | сумма ряда элементов (т.е. наш конечный ответ на задачу) |
| term | struct (Complex) | значение текущего элемента ряда |
| z_pow | struct (Complex) | комплексное число z в степени n |
| tolerance | double | точность вычисления комплексной экспоненты (12 знаков после запятой) |
| n | int | номер текущего члена; степень, в которую возводим z = число, факториал которого считаем (по формуле комплексной экспоненты) |
| term_magnitude | double | текущая точность вычислений на текущем члене ряда = модуль комплексного числа = длина вектора |
| z_user | struct (Complex) | комплексное число z, вводимое с клавиатурыx, экспоненту которого мы считаем |
| result_user | struct (Complex) | результирующая переменная, вычисленная экспонента по z |

#### Код программы

``` c
#include <stdio.h>
#include <math.h>

struct Complex{ // основная структура для комплексного числа 
    double real;
    double imag;
} ;

struct Complex complex_add(struct Complex a, struct Complex b){ // Сложение двух комплексных чисел
    struct Complex result;
    result.real = a.real + b.real;
    result.imag = a.imag + b.imag;
    return result;
}
struct Complex complex_multiply(struct Complex a, struct Complex b){ // Умножение двух комплексных чисел(= возведение z в степень)
    struct Complex result;
    result.real = a.real * b.real - a.imag * b.imag;
    result.imag = a.real * b.imag + a.imag * b.real;
    return result;
}
struct Complex complex_scale(struct Complex a, double scalar){ // Умножение комплексного числа на скаляр (вещественное число)
    struct Complex result;
    result.real = a.real * scalar;
    result.imag = a.imag * scalar;
    return result;
}
double factorial(int x){ // Вычисление факториала x!
    long double result = 1.0;
    for (int i = 2; i <= x; i++) {
        result *= i;
    }
    return result;
}
void print_complex(struct Complex z){ // Вывод комплексного числа на экран
        printf("%.6f + %.6fi", z.real, fabs(z.imag));
}
 

struct Complex complex_exp(struct Complex z){ // ВЫЧИСЛЕНИЕ КОМПЛЕКСНОЙ ЭКСПОНЕНТЫ
    struct Complex sum;      // сумма ряда
    struct Complex term;     // текущий член ряда
    struct Complex z_pow;  // z в степени n
    
    double tolerance = 1e-12;  // точность вычислений (12 знаков после запятой)
    int n = 0;               // номер текущего члена
    
    // sum = 1 (нулевой член ряда: z⁰/0! = 1)
    sum.real = 1.0;
    sum.imag = 0.0;
    // term = 1 (первый член для начала цикла)
    term.real = 1.0;
    term.imag = 0.0;
    
    // z_pow = 1 (z⁰ = 1)
    z_pow.real = 1.0;
    z_pow.imag = 0.0;
    
    printf("Вычисление exp(z) для z = ");
    print_complex(z);
    printf("\n");
    printf("n\tчлен ряда\t\tтекущая сумма\n");
    printf("------------\n");
    
    // Выводим нулевой член (1)
    printf("0\t1.000000\t\t");
    print_complex(sum);
    printf("\n");
    
    // Вычисляем ряд пока члены ряда значимы
    while (n < 100) {  // ограничение на количество итераций
        n++;
        
        // Вычисляем z^n = z^(n-1) × z
        z_pow = complex_multiply(z_pow, z);
        
        // Вычисляем член ряда: z^n / n!
        term = complex_scale(z_pow, 1.0 / factorial(n));
        
        // Добавляем член ряда к сумме
        sum = complex_add(sum, term);
        
        // Выводим информацию о текущей итерации
        printf("%d\t", n);
        print_complex(term);
        printf("\t");
        print_complex(sum);
        printf("\n");
        
        // если член ряда стал очень маленьким, он не сильно повлияет на сумму => останавливаемся
        double term_magnitude = sqrt(term.real * term.real + term.imag * term.imag); // = "длина" комплексного числа, фактически модуль!
        if (term_magnitude < tolerance) {
            printf("Достигнута точность в 12 знаков при n = %d\n", n);
            break;
        }
    }
    return sum;
}

int main() {
    /* будем вводить такие значения:
     ТЕСТ 1: exp(0 + 0i) 
     ТЕСТ 2: exp(1 + 0i) = e (проверка на действительном числе)
     ТЕСТ 3: exp(1 + 1i)
     ТЕСТ 4: exp(1 + 2i) */

    struct Complex z_user;  // ввод с клавиатуры
    
    printf("Введите действительную часть: ");
    scanf("%lf", &z_user.real);
    printf("Введите мнимую часть: ");
    scanf("%lf", &z_user.imag);
    
    struct Complex result_user = complex_exp(z_user);

    printf("exp(");
    print_complex(z_user);
    printf(") = ");
    print_complex(result_user);
    
    return 0;
}
```

#### Результат работы программы

z = 0 + 0i
<img width="1271" height="407" alt="6e5939c50fc19d01ea264ef00b6a1b20" src="https://github.com/user-attachments/assets/75ed90fa-aff5-4cc1-bbe9-d042e6090e91" />

z = 1 + 0i
<img width="1054" height="838" alt="63cd57211d5aec5ccb38a7ec25517ebf" src="https://github.com/user-attachments/assets/d5e46e94-8390-485a-90ea-2538dc18ace0" />

z = 1 + 1i
<img width="962" height="940" alt="9459ec5bfe8c8ec1834b1b2d690eb090" src="https://github.com/user-attachments/assets/c12db72b-1da6-473a-a569-e67c6f92d2cf" />

z = 1 + 2i
<img width="962" height="940" alt="9459ec5bfe8c8ec1834b1b2d690eb090" src="https://github.com/user-attachments/assets/457e6ba7-4d6a-467c-b42d-d897c6dcb1c2" /

### Выполнила: Жукова София 1об ПОО
