---@class SumLines: ISumLines
local SumLines = {}
SumLines.__index = SumLines

---@param lineStrategy ILineFeed
---@return SumLines
function SumLines:new(lineStrategy)
    ---@class SumLines
    local ins = setmetatable({}, self)
    ins.lineStrategy = lineStrategy
    return ins
end

---@param filename string
---@return number
function SumLines:process(filename)
    local h = io.open(filename, "r")
    assert(h, tostring(filename) .. " could not be found")

    local count = 0
    for line in h:lines() do
        local num = self.lineStrategy:process(line)
        -- print(num)
        count = count + num
    end
    h:close()
    return count
end

return SumLines