#!/usr/bin/env luajit
local infile = arg[1] or "input.txt"

local f = assert(io.open(infile))

local get_stacks = function(lines)
    local stacks = {}
    for i=2, #lines[1], 4 do
        -- [A] [B] [C] [D]
        --  ^   ^   ^   ^
        local stack = {}
        for ln = #lines, 1, -1 do
            local crate = lines[ln]:sub(i,i)
            if crate ~= " " then table.insert(stack, crate) end
        end
        table.insert(stacks, stack)
    end
    return stacks
end

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
        -- PART 1
        for i=1, move do
            table.insert(stacks_p1[to], table.remove(stacks_p1[from]))
        end
        -- PART 2
        local n = #stacks_p2[from] - move + 1
        for i=1, move do
            table.insert(stacks_p2[to], table.remove(stacks_p2[from], n))
        end
    end
end

local tops = function(stacks)
    local s = ""
    for i,v in ipairs(stacks) do s=s..v[#v] end
    return s
end

io.write("[PART ONE] (crate on top of each stack): ", tops(stacks_p1), "\n")
io.write("[PART ONE] (crate on top of each stack): ", tops(stacks_p2), "\n")

f:close()
