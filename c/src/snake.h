#include <stdbool.h>
#include <stddef.h>

#ifndef SNAKE_H
#define SNAKE_H

#define MAX_SNAKE_LENGTH 500

typedef enum { UP, DOWN, LEFT, RIGHT } Direction;

typedef struct {
    int x;
    int y;
} Position;

bool position_equals(Position a, Position b);

typedef struct {
    int length;
    Position body[MAX_SNAKE_LENGTH];
    Direction direction;
} Snake;

Snake *make_snake(Position start_pos);
void move_snake(Snake *s, Direction dir);
bool snake_contains_position(Snake *s, Position pos);
bool snake_collided_with_self(Snake *s);
bool snake_collided_with_wall(Snake *s, int max_x, int max_y);

#endif   // SNAKE_H