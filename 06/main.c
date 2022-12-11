#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#define ALPHABET_SIZE 26

int solve(FILE *f, const char *part, int seq_size) {
    rewind(f);
    bool found = false;
    while (!found) {
        found = true;
        char chars_seen[ALPHABET_SIZE] = {0};
        for (int i = 0; i < seq_size; ++i) {
            char c = fgetc(f);
            if (c == EOF) return fprintf(stderr, "INVALID INPUT\n"), 1;
            if (chars_seen[c - 'a']++ > 0) {
                fseek(f, -i, SEEK_CUR);
                found = false;
                break;
            }
        }
    }
    printf("[PART %s]: %ld\n", part, ftell(f));
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
