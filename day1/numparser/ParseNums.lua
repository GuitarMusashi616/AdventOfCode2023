---@class IterNums
local IterNums = require "numparser/IterNums"

---@class ParseNums: Parser
local ParseNums = {}
ParseNums.__index = ParseNums

---@return ParseNums
function ParseNums:new()
    local ins = setmetatable({}, self)
    return ins
end

---@param line string
---@return Iterator # makes and returns an iteration of all the numbers in the line
function ParseNums:parse(line)  --luacheck: ignore
    return IterNums:new(line)
end

return ParseNums