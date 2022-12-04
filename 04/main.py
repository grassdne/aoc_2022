#!/usr/bin/env python3
import sys

def is_superrange(a, b):
    """ a starts before b starts and ends after b ends """
    return a[0] <= b[0] and a[1] >= b[1]

def is_overlap(a, b):
    """ a ends after b starts but starts before b ends """
    return a[1] >= b[0] and a[0] <= b[1] 

def main():
    f = open("input.txt" if len(sys.argv) < 2 else sys.argv[1])
    n_superrange = 0
    n_overlaps = 0
    for line in f:
        # "a-b,c-d" -> [[a, b], [c, d]]
        pair = [[int(x) for x in elf.split('-')] for elf in line.split(',')]
        if is_superrange(pair[0], pair[1]) or is_superrange(pair[1], pair[0]):
            n_superrange += 1
        
        if is_overlap(pair[0], pair[1]) or is_overlap(pair[1], pair[0]):
            n_overlaps += 1

    print("[PART ONE] # assignment pairs where one range fully contains other:", n_superrange)
    print("[PART TWO] # assinment pairs where the ranges overlap:", n_overlaps)

if __name__ == '__main__':
    main()
