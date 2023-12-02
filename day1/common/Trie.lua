---@class Trie: ITrie
local Trie = {}
Trie.__index = Trie

---@return Trie
function Trie:new()
    ---@class Trie
    local ins = setmetatable({}, self)
    ins.trie = {}
    return ins
end

---@param map table<string, any>
---@return Trie
function Trie:fromMap(map)
    ---@class Trie
    local ins = setmetatable({}, self)
    ins.trie = {}
    for word, num in pairs(map) do
        ins:insert(word, num)
    end
    return ins
end

---@param word string
---@param value any
function Trie:insert(word, value)  --luacheck: ignore
    local res = self.trie
    for i = 1, #word-1 do
        local chr = word:sub(i, i)
        if res[chr] == nil then
            res[chr] = {}
        end
        res = res[chr]
    end
    res[word:sub(#word, #word)] = value
end

function Trie:checkFor(needle)
    local res = self.trie
    for i = 1, #needle do
        local chr = needle:sub(i, i)

        if res[chr] == nil then
            return
        end

        if type(res[chr]) ~= "table" then
            return res[chr]
        end

        res = res[chr]
    end
    return res
end

return Trie