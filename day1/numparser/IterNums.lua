---@class IterNums: Iterator
local IterNums = {}
IterNums.__index = IterNums

---@param line string
---@return IterNums
function IterNums:new(line)
    ---@class IterNums
    local ins = setmetatable({}, self)
    ins.line = line
    ins.index = 0
    return ins
end

---@return string?
function IterNums:__call()
    local nextLetter
    repeat
        self.index = self.index + 1
        nextLetter = self.line:sub(self.index, self.index)
    until(nextLetter=="" or tonumber(nextLetter)~=nil)
    if (nextLetter=="") then
        return
    end
    return nextLetter
end

return IterNums