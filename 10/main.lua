#!/usr/bin/luajit

local ops = {}
for ln in io.lines((assert(arg[1], "expected input file argument"))) do
    if ln == "noop" then table.insert(ops, 0)
    else
        local n = assert(ln:match("addx (.*)"), ln)
        table.insert(ops, 0)
        table.insert(ops, n)
    end
end

local reg = {[0] = 1}
for i,v in ipairs(ops) do
    reg[i] = reg[i-1] + v
end

local str = {}
for i = 1, #ops do str[i] = i * reg[i-1] end

io.write("[PART ONE]: ", str[20] + str[60] + str[100] + str[140] + str[180] + str[220], "\n")

local pixels = {}
local i = 1
for r = 1, 6 do
    pixels[r] = {}
    for c = 0, 39 do
        print("Cycle:", i, "reg:", reg[i-1])
        pixels[r][c] = math.abs(reg[i-1] - c) <= 1
        i=i+1
    end
end

for _,row in ipairs(pixels) do
    for i = 0, 39 do
        io.write(row[i] and "#" or ".")
    end
    io.write "\n"
end
