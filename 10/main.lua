#!/usr/bin/luajit

local deltas = {1}
for ln in io.lines((assert(arg[1], "expected input file argument"))) do
    if ln == "noop" then table.insert(deltas, 0)
    else
        local n = assert(ln:match("addx (.*)"), ln)
        table.insert(deltas, 0)
        table.insert(deltas, tonumber(n))
    end
end

local register = {}
for i,delta in ipairs(deltas) do
    register[i] = (register[i-1] or 0) + delta
end

local strength = {}
for i = 1, #deltas do strength[i] = i * register[i] end

io.write("[PART ONE]: ", strength[20] + strength[60] + strength[100] + strength[140] + strength[180] + strength[220], "\n")

local pixels = {}
local i = 1
for r = 1, 6 do
    pixels[r] = {}
    for c = 0, 39 do
        pixels[r][c] = math.abs(register[i] - c) <= 1
        i=i+1
    end
end

print("[PART TWO]:")
for _,row in ipairs(pixels) do
    for i = 0, 39 do
        io.write(row[i] and "#" or ".")
    end
    io.write "\n"
end
