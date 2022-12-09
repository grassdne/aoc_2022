#!/usr/bin/env luajit

imap = function(a, f)
    local t = {}
    for i,v in ipairs(a) do t[i] = f(v) end
    return t
end
reverse = function(a)
    local t = {}
    for i=#a, 1, -1 do t[#t+1] = a[i] end
    return t
end

local get_stacks = function(lines)
    local stacks = {}
    for i=2, #lines[1], 4 do
        -- [A] [B] [C] [D]
        --  ^   ^   ^   ^
        table.insert(stacks, imap(reverse(lines), function(ln) return ln:sub(i,i) ~= " " and ln:sub(i,i) or nil end))
    end
    return stacks
end

local infile = arg[1] or "input.txt"
local f = assert(io.open(infile))

local items = {}
for ln in f:lines() do
    -- Stop at number headers
    if ln:find("^ 1") then break end
    table.insert(items, ln)
end

local stacks_p1 = get_stacks(items)
local stacks_p2 = get_stacks(items)

for ln in f:lines() do
    local move, from, to = ln:match("move (%d*) from (%d*) to (%d*)")
    if move then
        move, from, to = tonumber(move), tonumber(from), tonumber(to)
        local n = #stacks_p2[from] - move + 1
        for i=1, move do
            table.insert(stacks_p1[to], table.remove(stacks_p1[from]))
            table.insert(stacks_p2[to], table.remove(stacks_p2[from], n))
        end
    end
end

io.write("[PART ONE] (crate on top of each stack): ", table.concat(imap(stacks_p1, function(a) return a[#a] end)), "\n")
io.write("[PART TWO] (crate on top of each stack): ", table.concat(imap(stacks_p2, function(a) return a[#a] end)), "\n")

f:close()
