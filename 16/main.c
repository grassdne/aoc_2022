#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <assert.h>

#define ALPH_SIZE 26
#define MAX_CONNECTED 8

typedef struct {
    int to;
    int dist;
} Pathway;

#define PATHWAY_CAP 64

typedef enum {
    VALVE_NULL=0,
    VALVE_CLOSED,
    VALVE_OPEN,
} ValveState;

typedef struct {
    int flow_rate;
    ValveState state;
    int *connected;
    Pathway *pathways;
    int num_pathways;
} Valve;

#define VALVE_EXISTS()

#define VALVEPOOL_SIZE (ALPH_SIZE * ALPH_SIZE + 1)
typedef Valve ValvePool[VALVEPOOL_SIZE];

#define FOR_EACH_VALVE(vp, id) for (int id = 0; id < VALVEPOOL_SIZE; id++) if (vp[id].state)

int get_id(char *s) {
    // (very advanced hashing algorithm)
    return (s[0]-'A') * ALPH_SIZE + (s[1]-'A') + 1;
}

bool parse_valve(FILE *f, ValvePool vp) {
    char item[2];
    int flow_rate;
    int res = fscanf(f, "Valve %2c has flow rate=%d; %*s %*s to %*s ", item, &flow_rate);
    if (res < 2) return false;

    int id = get_id(item);
    char s[64] = {0};
    if (fgets(s, sizeof(s), f) == 0)
        fprintf(stderr, "expected tunnel list"), exit(1);

    char *p = s;
    vp[id].connected = calloc(MAX_CONNECTED, sizeof(int));
    for (int i = 0; *p != '\n' && *p != 0; i++) {
        assert(i < MAX_CONNECTED);
        vp[id].connected[i] = get_id(p);
        p += 4;
    }

    vp[id].flow_rate = flow_rate;
    vp[id].state = VALVE_CLOSED;

    return true;
}

// TODO: consider dynamic array
#define STACK_CAPACITY 64
typedef struct {
    int arr[STACK_CAPACITY];
    int len;
} Stack;

static inline void push(Stack *stack, int v) {
    assert(stack->len < STACK_CAPACITY);
    stack->arr[stack->len++] = v;
}
static inline int pop(Stack *stack) {
    assert(stack->len > 0);
    return stack->arr[--stack->len];
}

void explore(ValvePool vp, int valve) {
    bool explored[VALVEPOOL_SIZE] = {0};
    Stack last_explored = {0};
    Stack tmp_explored = {0};
    explored[valve] = true;
    push(&last_explored, valve);

    int dist = 0;
    while (last_explored.len > 0) {
        ++dist;
        while (last_explored.len > 0) {
            int id = pop(&last_explored);
            for (int *to = vp[id].connected; *to; to++) {
                if (!explored[*to] && vp[*to].flow_rate > 0) {
                    vp[valve].pathways[vp[valve].num_pathways++] = (Pathway){ .to = *to, .dist = dist };
                }
                if (!explored[*to]) push(&tmp_explored, *to);
                explored[*to] = true;
            }
        }
        // I wonder if the compiler can convert this to a memcpy
        while (tmp_explored.len > 0)
            push(&last_explored, pop(&tmp_explored));
    }
}

void generate_pathways(ValvePool vp, int *valve_ids, int nvalves) {
    for (int i = 0; i < nvalves; i++) {
        int id = valve_ids[i];
        vp[id].pathways = calloc(PATHWAY_CAP, sizeof(Pathway));
        explore(vp, id);
    }
}

static inline int max(int a, int b) {
    return a > b ? a : b;
}

// I really can't be bothered to abstract these two into a generic function

int optimal_pressure_single(ValvePool vp, int id, int time_left) {
    int greatest = 0;
    const int n = vp[id].num_pathways;
    for (int i = 0; i < n; i++) {
        Pathway path = vp[id].pathways[i];
        if (path.dist < time_left && vp[path.to].state == VALVE_CLOSED) {
            vp[path.to].state = VALVE_OPEN;
            int time = time_left - path.dist - 1;
            int v = optimal_pressure_single(vp, path.to, time) + time * vp[path.to].flow_rate;
            greatest = max(greatest, v);
            vp[path.to].state = VALVE_CLOSED;
        }
    }
    return greatest;
}

int optimal_pressure_double(ValvePool vp, int loc_a, int loc_b, int time_a, int time_b) {
    int greatest = 0;
    const int n = vp[loc_a].num_pathways;
    const int m = vp[loc_b].num_pathways;
    for (int i = 0; i < n; i++) {
        Pathway path_a = vp[loc_a].pathways[i];
        int new_time_a = time_a - path_a.dist - 1;
        int flow_a = new_time_a * vp[path_a.to].flow_rate;

        if (path_a.dist < time_a && vp[path_a.to].state == VALVE_CLOSED) {
            vp[path_a.to].state = VALVE_OPEN;
            bool loc_b_path_found = false;
            for (int j = 0; j < m; j++) {
                Pathway path_b = vp[loc_b].pathways[j];
                if (path_b.dist < time_b && vp[path_b.to].state == VALVE_CLOSED) {
                    loc_b_path_found = true;
                    vp[path_b.to].state = VALVE_OPEN;
                    int new_time_b = time_b - path_b.dist - 1;
                    int flow_b = new_time_b * vp[path_b.to].flow_rate;
                    int v = optimal_pressure_double(vp, path_a.to, path_b.to, new_time_a, new_time_b)
                        + flow_b + flow_a;
                    greatest = max(greatest, v);

                    vp[path_b.to].state = VALVE_CLOSED;
                }
            }
            if (!loc_b_path_found) {
                // go back to single
                greatest = max(greatest, flow_a + optimal_pressure_single(vp, path_a.to, new_time_a));
            }

            vp[path_a.to].state = VALVE_CLOSED;
        }
    }
    return greatest;
}


int main(int argc, char **argv) {
    if (argc < 2) return fprintf(stderr, "expected input file argument\n"), 1;

    ValvePool vp = {0};
    FILE *f = fopen(argv[1], "r");
    if (f == NULL) return fprintf(stderr, "unable to open input file\n"),    1;

    while (parse_valve(f, vp));
    fclose(f);

    int valve_ids[64] = {0};
    int i = 0;
    FOR_EACH_VALVE(vp, id) {
        valve_ids[i++] = id;
    }

    generate_pathways(vp, valve_ids, i);

    //dump_pathways(vp);

    printf("[PART ONE]: %d\n", optimal_pressure_single(vp, get_id("AA"), 30));
    printf("[PART TWO]: %d\n", optimal_pressure_double(vp, get_id("AA"), get_id("AA"), 26, 26));

    FOR_EACH_VALVE(vp, id) free(vp[id].pathways), free(vp[id].connected);
    return 0;
}
