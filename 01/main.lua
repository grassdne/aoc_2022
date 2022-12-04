#!/usr/bin/env lua
input_file = arg[1] or "input.txt"

local elves = {}

local i = 1
for n in io.lines(input_file) do
    if n == "" then i=i+1
    else elves[i] = (elves[i] or 0) + tonumber(n)
    end
end
table.sort(elves)

io.write("[PART ONE] How many Calories the Elf with the most snacks carries: ", elves[#elves], "\n")
io.write("[PART TWO] How many Calories the top three prepared Elves carry: ", elves[#elves] + elves[#elves-1] + elves[#elves-2], "\n")
