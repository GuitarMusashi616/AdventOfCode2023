---@class IterWordNums
local IterWordNums = require "numparser/IterWordNums"
local Trie = require "common/Trie"

---@class ParseWordNums: Parser
local ParseWordNums = {}
ParseWordNums.__index = ParseWordNums

---@param wordNumMap table<string, number>
---@return ParseWordNums
function ParseWordNums:new(wordNumMap)
    ---@class ParseWordNums
    local ins = setmetatable({}, self)
    ins.wordNumTrie = Trie:fromMap(wordNumMap)
    return ins
end

---@param line string
---@return Iterator
function ParseWordNums:parse(line)  --luacheck: ignore
    return IterWordNums:new(line, self.wordNumTrie)
end

return ParseWordNums