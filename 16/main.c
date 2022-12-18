#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <assert.h>

#define ALPH_SIZE 26
#define MAX_CONNECTED 8
#define MAX_WORKERS 2

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

void generate_pathways(ValvePool vp) {
    FOR_EACH_VALVE(vp, id) {
        vp[id].pathways = calloc(PATHWAY_CAP, sizeof(Pathway));
        explore(vp, id);
    }
}

static inline int max(int a, int b) {
    return a > b ? a : b;
}

static inline int sum(int *a, int n) {
    int sum = 0;
    for (int i = 0; i < n; i++)
        sum += a[i];
    return sum;
}

static void follow_pathway(const ValvePool vp, const Pathway *paths, const int *times, int *out_time, int *out_flow, const int num_workers) {
    for (int i = 0; i < num_workers; i++) {
        out_time[i] = times[i] - paths[i].dist - 1;
        out_flow[i] = out_time[i] * vp[paths[i].to].flow_rate;
    }
}

static bool path_usable(const ValvePool vp, const Pathway path, const int cur_time) {
    return path.dist < cur_time && vp[path.to].state == VALVE_CLOSED;
}

#define try_all_paths(vp, locs, times, num_workers) \
    _try_all_paths((vp), (locs), (times), (num_workers), 0, (Pathway[MAX_WORKERS]){})

static int _try_all_paths(ValvePool vp, const int *locs, const int *times, int num_workers, int worker, Pathway *paths) {
    const int num_pathways = vp[locs[worker]].num_pathways;
    int greatest = 0;
    for (int i = 0; i < num_pathways; i++) {
        paths[worker] = vp[locs[worker]].pathways[i];
        if (path_usable(vp, paths[worker], times[worker])) {
            vp[paths[worker].to].state = VALVE_OPEN;
            int found = 0;
            if (worker < num_workers-1) {
                // Try all paths for next worker
                found = _try_all_paths(vp, locs, times, num_workers, worker+1, paths);
            }
            if (found) {
                greatest = max(greatest, found);
            }
            else {
                num_workers = worker+1;
                int new_times[MAX_WORKERS]; 
                int new_flows[MAX_WORKERS]; 
                int new_locs[MAX_WORKERS]; 
                for (int i = 0; i < num_workers; i++) new_locs[i] = paths[i].to;
                follow_pathway(vp, paths, times, new_times, new_flows, num_workers);
                greatest = max(greatest, try_all_paths(vp, new_locs, new_times, num_workers)
                         + sum(new_flows, num_workers));
            }

            vp[paths[worker].to].state = VALVE_CLOSED;
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

    generate_pathways(vp);

    int locs[] = {get_id("AA"), get_id("AA")};
    int p1_start[] = {30};
    int p2_start[] = { 26, 26 };
    printf("[PART ONE]: %d\n", try_all_paths(vp, locs, p1_start, 1));
    printf("[PART TWO]: %d\n", try_all_paths(vp, locs, p2_start, 2));

    FOR_EACH_VALVE(vp, id) free(vp[id].pathways), free(vp[id].connected);
    return 0;
}
