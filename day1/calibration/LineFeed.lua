---Uses numbers parsed from the line to sum up the line
---@class LineFeed: ILineFeed
local LineFeed = {}
LineFeed.__index = LineFeed

---@param parser Parser
---@return LineFeed
function LineFeed:new(parser)
    ---@class LineFeed
    local ins = setmetatable({}, self)
    ins.parser = parser
    return ins
end

---@private
---@param table table
---@param line string
function LineFeed:printNumbers(table, line)  --luacheck: ignore
    local msg = ""
    for i = 1, #table-1 do
        msg = msg .. tostring(table[i])
        msg = msg .. ", "
    end
    msg = msg .. table[#table]
    print(line, " --------> ", msg)
end

---@private
---@param line string
---@return string[]
function LineFeed:collectNumbers(line)
    local numbers = {}
    for number in self.parser:parse(line) do
        numbers[#numbers + 1] = number
    end
    self:printNumbers(numbers, line)
    return numbers
end

---@private
---@param line string
---@return string
function LineFeed:getFirstLastNumber(line)
    local numbers = self:collectNumbers(line)
    assert(#numbers >= 1, "Line does not contain at least 1 number: " .. tostring(line))
    return numbers[1] .. numbers[#numbers]
end

---@param line string
---@return number?
function LineFeed:process(line)
    local numStr = self:getFirstLastNumber(line)
    return tonumber(numStr)
end


return LineFeed