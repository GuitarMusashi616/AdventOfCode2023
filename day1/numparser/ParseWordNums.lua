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

-- ---@param wordNumMap table<string, number>
-- ---@return table
-- function ParseWordNums:makeTrie(wordNumMap)
--     local trie = {}
--     for word, num in pairs(wordNumMap) do
--         self:insertWord(word, num, trie)
--     end
--     return trie
-- end

-- ---@param word string
-- ---@param number number
-- ---@param trie table
-- function ParseWordNums:insertWord(word, number, trie)  --luacheck: ignore
--     local res = trie
--     for i = 1, #word-1 do
--         local chr = word:sub(i, i)
--         if res[chr] == nil then
--             res[chr] = {}
--         end
--         res = res[chr]
--     end
--     res[word:sub(#word, #word)] = number
-- end

---@param line string
---@return Iterator # makes and returns an iteration of all the numbers in the line
function ParseWordNums:parse(line)  --luacheck: ignore
    return IterWordNums:new(line, self.wordNumTrie)
end

return ParseWordNums