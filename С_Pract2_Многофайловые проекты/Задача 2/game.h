#ifndef GAME_H
#define GAME_H

// Генерация случайного числа в диапазоне [lower, upper]
int generateNumber(int lower, int upper);

// Проверка догадки: возвращает -1, если guess меньше target, 0, если равны, 1, если guess больше target
int checkGuess(int target, int guess);

#endif // GAME_H