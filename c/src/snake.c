#include "snake.h"

#include <stdbool.h>
#include <stddef.h>
#include <stdlib.h>

bool position_equals(Position a, Position b) {
    return a.x == b.x && a.y == b.y;
}

Snake *make_snake(Position start) {
    Snake *s = malloc(sizeof(Snake));
    if (s == NULL) {
        return NULL;
    }

    s->length = 1;
    s->body[0] = start;
    s->direction = RIGHT;

    return s;
}

void move_snake(Snake *s, Direction dir) {
    for (int i = s->length - 1; i > 0; i--) {
        s->body[i] = s->body[i - 1];
    }

    switch (dir) {
        case UP:
            s->body[0].y--;
            break;
        case DOWN:
            s->body[0].y++;
            break;
        case LEFT:
            s->body[0].x--;
            break;
        case RIGHT:
            s->body[0].x++;
            break;
    }

    s->direction = dir;
}

bool snake_contains_position(Snake *s, Position pos) {
    for (int i = 0; i < s->length; i++) {
        if (position_equals(s->body[i], pos)) {
            return true;
        }
    }

    return false;
}

bool snake_collided_with_self(Snake *s) {
    Position head = s->body[0];
    for (int i = 1; i < s->length; i++) {
        if (position_equals(s->body[i], head)) {
            return true;
        }
    }

    return false;
}

bool snake_collided_with_wall(Snake *s, int max_x, int max_y) {
    Position head = s->body[0];
    return head.x <= 0 || head.x >= max_x - 1 || head.y <= 0 ||
           head.y >= max_y - 1;
}