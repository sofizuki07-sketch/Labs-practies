#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "game.h"

int main(void) {
    int lower = 1, upper = 100;
    int target, guess, result;

    // Инициализация генератора случайных чисел
    srand(time(NULL));
    target = generateNumber(lower, upper);
    printf("Угадайте число от %d до %d:\n", lower, upper);
    do {
        printf("Введите вашу догадку: ");
        scanf("%d", &guess);
        result = checkGuess(target, guess);
        if (result == -1) {
            printf("Слишком мало!\n");
        } else if (result == 1) {
            printf("Слишком много!\n");
        }
    } while (result != 0);
    
    printf("Поздравляем! Вы угадали число %d.\n", target);
    return 0;
}
