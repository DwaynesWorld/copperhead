#include <ncurses.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#include "snake.h"

int main() {
    // Initialize ncurses.
    initscr();               // Start curses mode.
    cbreak();                // Disable line buffering.
    noecho();                // Do not echo pressed keys.
    curs_set(FALSE);         // Hide the cursor.
    keypad(stdscr, TRUE);    // Enable arrow keys.
    nodelay(stdscr, TRUE);   // Non-blocking input.

    // Seed the random number generator.
    srand(time(NULL));

    int max_y, max_x;
    getmaxyx(stdscr, max_y, max_x);

    // Initialize the snake in the center of the screen.
    Position start = {max_x / 2, max_y / 2};
    Snake *snake = make_snake(start);

    // Place the first food item randomly.
    Position food = {rand() % max_x, rand() % max_y};

    // Main game loop.
    while (true) {
        clear();   // Clear the screen.

        // Draw the snake.
        for (int i = 0; i < snake->length; i++) {
            mvaddch(snake->body[i].y, snake->body[i].x, 'o');
        }

        // Draw the food.
        mvaddch(food.y, food.x, 'x');

        // Refresh the screen.
        refresh();

        // Get the next direction from the user.
        Direction next_direction = snake->direction;
        int ch = getch();
        switch (ch) {
            case KEY_UP:
                if (snake->direction != DOWN)
                    next_direction = UP;
                break;
            case KEY_DOWN:
                if (snake->direction != UP)
                    next_direction = DOWN;
                break;
            case KEY_LEFT:
                if (snake->direction != RIGHT)
                    next_direction = LEFT;
                break;
            case KEY_RIGHT:
                if (snake->direction != LEFT)
                    next_direction = RIGHT;
                break;
            case 'q':
                endwin();
                return 0;
        }

        // Check if the snake ate the food.
        if (position_equals(snake->body[0], food)) {
            snake->length++;
            food = (Position){rand() % max_x, rand() % max_y};
        }

        // Move the snake in the next direction.
        move_snake(snake, next_direction);

        // Check for collisions.
        if (snake_collided_with_wall(snake, max_x, max_y) ||
            snake_collided_with_self(snake)) {
            break;
        }

        // Sleep for a short amount of time.
        struct timespec ts = {0, 100000000};
        nanosleep(&ts, NULL);
    }

    clear();
    mvprintw(max_y / 2, (max_x - 10) / 2, "Game Over!");
    refresh();

    struct timespec ts = {2, 100000000};
    nanosleep(&ts, NULL);

    // Clean up ncurses.
    endwin();
    return 0;
}