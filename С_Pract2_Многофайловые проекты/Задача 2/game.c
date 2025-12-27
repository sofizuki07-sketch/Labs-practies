#include <stdlib.h>
#include "game.h"

// Генерация случайного числа в диапазоне [lower, upper]
int generateNumber(int lower, int upper) {
    return lower + rand() % (upper - lower + 1);
}

// Сравнение target и guess
int checkGuess(int target, int guess) {
    if (guess == target) {
        return 0;
    } else if (guess < target) {
        return -1;
    } else {
        return 1;
    }
}
