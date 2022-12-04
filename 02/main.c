#include <stdio.h>
#include <stdlib.h>

typedef enum {
    OPPONENT_ROCK = 'A',
    OPPONENT_PAPER = 'B',
    OPPONENT_SCISSOR = 'C',
} OpponentPlay;

typedef enum {
    RESPOND_ROCK = 'X',
    RESPOND_PAPER = 'Y',
    RESPOND_SCISSOR = 'Z',
} RespondPlay;

typedef enum {
    RESPOND_LOSE = 'X',
    RESPOND_DRAW = 'Y',
    RESPOND_WIN = 'Z',
} RespondOutcome;

#define SCORE_LOSS 0
#define SCORE_DRAW 3
#define SCORE_WIN 6

const RespondPlay SHAPE_SCORE[256] = {
    [RESPOND_ROCK] = 1,
    [RESPOND_PAPER] = 2,
    [RESPOND_SCISSOR] = 3,
};

static RespondPlay win_response(OpponentPlay other) {
    switch (other) {
        case OPPONENT_ROCK:    return RESPOND_PAPER;
        case OPPONENT_PAPER:   return RESPOND_SCISSOR;
        case OPPONENT_SCISSOR: return RESPOND_ROCK;
    }__builtin_unreachable();
}

static RespondPlay tie_response(OpponentPlay other) {
    switch (other) {
        case OPPONENT_ROCK:    return RESPOND_ROCK;
        case OPPONENT_PAPER:   return RESPOND_PAPER;
        case OPPONENT_SCISSOR: return RESPOND_SCISSOR;
    }__builtin_unreachable();
}

static RespondPlay lose_response(OpponentPlay other) {
    switch (other) {
        case OPPONENT_PAPER:   return RESPOND_ROCK;
        case OPPONENT_SCISSOR: return RESPOND_PAPER;
        case OPPONENT_ROCK:    return RESPOND_SCISSOR;
    }__builtin_unreachable();
}

static RespondPlay response_shape(OpponentPlay other, RespondOutcome mine) {
    switch (mine) {
    case RESPOND_LOSE: return lose_response(other);
    case RESPOND_DRAW: return tie_response(other);
    case RESPOND_WIN:  return win_response(other);
    }__builtin_unreachable();
}

static int outcome_score(RespondOutcome mine) {
    switch (mine) {
    case RESPOND_LOSE: return SCORE_LOSS;
    case RESPOND_DRAW: return SCORE_DRAW;
    case RESPOND_WIN:  return SCORE_WIN;
    }__builtin_unreachable();
}

int main(void) {
    FILE *f = fopen("input.txt", "r");
    if (f == NULL) fprintf(stderr, "unable to open input file\n"), exit(1);
    char line[256];
    { // PART ONE
        int score = 0;
        while (fgets(line, sizeof(line), f) != NULL) {
            const OpponentPlay other = line[0];
            const RespondPlay mine = line[2];

            score += (mine == win_response(other)) * SCORE_WIN
                   + (mine == tie_response(other)) * SCORE_DRAW
                   + SHAPE_SCORE[mine];

        }

        printf("[PART ONE] Total score: %d\n", score);
    }

    fseek(f, 0L, SEEK_SET);

    { // PART TWO
        int score = 0;

        while (fgets(line, sizeof(line), f) != NULL) {
            const OpponentPlay other = line[0];
            const RespondOutcome mine = line[2];

            score += SHAPE_SCORE[response_shape(other, mine)] + (int)outcome_score(mine);
        }

        printf("[PART TWO] Total score: %d\n", score);
    }

    fclose(f);


    return 0;
}
