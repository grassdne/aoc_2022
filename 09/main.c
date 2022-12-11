#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <assert.h>

#define MAX_MOTIONS 2048

typedef struct { int x, y; } Vec2;

typedef enum {
    POS_EMPTY = 0,
    POS_VISITED,
} Pos;

typedef struct {
    Pos *items;
    Vec2 size;
    Vec2 start;
} Field;

typedef enum { RIGHT=1, UP, LEFT, DOWN } Direction;

Direction char_to_direction[] = {
    ['R'] = RIGHT,
    ['U'] = UP,
    ['L'] = LEFT,
    ['D'] = DOWN,
};

static Vec2 decx(Vec2 v) { --v.x; return v; }
static Vec2 incx(Vec2 v) { ++v.x; return v; }
static Vec2 decy(Vec2 v) { --v.y; return v; }
static Vec2 incy(Vec2 v) { ++v.y; return v; }

typedef Vec2 (*Action)(Vec2 v); 

Action dir_to_action[] = {
    [UP] = incy,
    [DOWN] = decy,
    [LEFT] = decx,
    [RIGHT] = incx,
};

typedef unsigned int Count;

typedef struct  {
    Direction direction;
    Count count;
} Motion;

bool get_instructions_from_file(Motion motions[MAX_MOTIONS], const char *fname) {
    FILE *f = fopen(fname, "r");
    if (f == NULL) return NULL;
    Motion *p = motions;
    char c;
    while(fscanf(f, "%c %d\n", &c, &p->count) == 2) {
        p->direction = char_to_direction[(int)c];
        p++;
    }
    fclose(f);
    return true;
}

typedef struct { int left, right, down, up; } FieldSize;
FieldSize get_required_field_size(Motion motions[MAX_MOTIONS]) {
    FieldSize size = {0};
    int current_x = 0;
    int current_y = 0;
    for (Motion *p = motions; p->direction != 0; p++) {
        switch (p->direction) {
            case RIGHT:
                current_x += p->count;
                break;
            case LEFT:
                current_x -= p->count;
                break;
            case UP:
                current_y += p->count;
                break;
            case DOWN:
                current_y -= p->count;
                break;
            default:
                assert(false && "unknown direction");
        }
        if (current_x < size.left) size.left = current_x;
        if (current_x > size.right) size.right = current_x;
        if (current_y < size.down) size.down = current_y;
        if (current_y > size.up) size.up = current_y;
    }
    return size;
}

Field malloc_field(FieldSize space) {
    Field field;
    field.start = (Vec2){-space.left, -space.down};
    field.size.x = space.right - space.left + 1;
    field.size.y = space.up - space.down + 1;
    field.items = calloc(field.size.x * field.size.y, sizeof(field.items[0]));
    return field;
}

void free_field(Field *field) {
    free(field->items);
    *field = (Field){0};
}

static Vec2 tail_step(Vec2 tail, Vec2 head) {
#define STEP(t, h) do { \
    if (t < h) ++t; else if (t > h) --t; \
} while(0)

    if (tail.x + 1 < head.x) {
        ++tail.x;
        STEP(tail.y, head.y);
    }
    if (tail.x - 1 > head.x) {
        --tail.x;
        STEP(tail.y, head.y);
    }
    if (tail.y + 1 < head.y) {
        ++tail.y;
        STEP(tail.x, head.x);
    }
    if (tail.y - 1 > head.y) {
        --tail.y;
        STEP(tail.x, head.x);
    }
    return tail;
#undef STEP
}

void perform_simulation_1(Field field, Motion motions[MAX_MOTIONS]) {
    Vec2 head = field.start;
    Vec2 tail = field.start;
    for (Motion *m = motions; m->direction; m++) {
        Action action = dir_to_action[m->direction];
        for (int i = 0; i < m->count; i++) {
            head = action(head);
            tail = tail_step(tail, head);
            field.items[tail.x + tail.y*field.size.x] = POS_VISITED;
        }
    }
}

#define N_KNOTS 10
void perform_simulation_2(Field field, Motion motions[MAX_MOTIONS]) {
    Vec2 xs[N_KNOTS];
    for (int i = 0; i < N_KNOTS; ++i) xs[i] = field.start;
    for (Motion *m = motions; m->direction; m++) {
        Action action = dir_to_action[m->direction];
        for (int i = 0; i < m->count; i++) {
            xs[0] = action(xs[0]);
            for (int i = 1; i < N_KNOTS; ++i) {
                xs[i] = tail_step(xs[i], xs[i-1]);
            }
            field.items[xs[N_KNOTS-1].x + xs[N_KNOTS-1].y*field.size.x] = POS_VISITED;
        }
    }
}

static int count_visited(Field field) {
    int count = 0;
    for (int i = 0; i < field.size.x * field.size.y; i++) {
        if (field.items[i] == POS_VISITED) {
            count++;
        }
    }
    return count;
}

static void clear_field(Field field) {
    for (int i = 0; i < field.size.x * field.size.y; i++) {
        field.items[i] = POS_EMPTY;
    }
}

int main(int argc, char **argv) {
    if (argc < 2) return fprintf(stderr, "expected input file\n"), 1;

    Motion motions[MAX_MOTIONS] = {0};

    if (!get_instructions_from_file(motions, argv[1]))
        return fprintf(stderr, "unable to process input file %s", argv[1]), 1;

    FieldSize fieldsize = get_required_field_size(motions);
    Field field = malloc_field(fieldsize);

    perform_simulation_1(field, motions);
    printf("[PART ONE]: %d\n", count_visited(field));

    clear_field(field);

    perform_simulation_2(field, motions);
    printf("[PART TWO]: %d\n", count_visited(field));

    free_field(&field);
    return 0;
}
