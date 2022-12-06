#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#define ALPH 26

int solve(FILE *f, const char *part, int N) {
    rewind(f);
    for (;;) next: {
        char items[ALPH] = {0};
        for (int i = 0; i < N; ++i) {
            char c = fgetc(f);
            if (c == EOF) return fprintf(stderr, "INVALID INPUT\n"), 1;
            if (++items[c - 'a'] > 1) {
                fseek(f, -i, SEEK_CUR);
                goto next;
            }
        }
        break;
    }
    printf("[PART %s] start-of-packet detected: %ld\n", part, ftell(f));
    return 0;
}

int main(int argc, char **argv) {
    const char *infile = argc > 1 ? argv[1] : "input.txt";
    FILE *f = fopen(infile, "r");
    if (f == NULL) fprintf(stderr, "unable to open input file %s\n", infile);
    
    if (solve(f, "ONE", 4) != 0) goto done;
    if (solve(f, "TWO", 14) != 0) goto done;
done:
    fclose(f);
    return 0;
}
