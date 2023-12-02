---@class IterWordNums: Iterator
local IterWordNums = {}
IterWordNums.__index = IterWordNums

---@param line string
---@param wordNumTrie ITrie
---@return IterWordNums
function IterWordNums:new(line, wordNumTrie)
    ---@class IterWordNums
    local ins = setmetatable({}, self)
    ins.line = line
    ins.processed = 0
    ins.index = 0
    ins.trie = wordNumTrie
    return ins
end

---@private
---@param index number
---@return number?
function IterWordNums:checkTrie(index)
    for i = index, #self.line do
        local needle = self.line:sub(index, i)
        local res = self.trie:checkFor(needle)
        if res == nil then
            return
        end
        if type(res) ~= "table" then
            return res
        end
    end
end

---@return string?
function IterWordNums:__call()
    local nextLetter
    while true do

        self.index = self.index + 1
        nextLetter = self.line:sub(self.index, self.index)

        -- return if it is a number
        if (tonumber(nextLetter)) then
            self.processed = self.index + 1
            return nextLetter
        end
        -- return if it is a word number
        local ans = self:checkTrie(self.index)
        if ans ~= nil then
            self.index = self.index + 1
            return tostring(ans)
        end
        -- return if no more chars in line
        if (nextLetter == "") then
            return
        end
    end
end

return IterWordNums