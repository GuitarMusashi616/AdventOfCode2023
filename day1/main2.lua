-- Part 2 solution
-- Only the parser instance has changed

local calibration = require "calibration"
local numparser = require "numparser"

local wordNumMap = {
    one = 1,
    two = 2,
    three = 3,
    four = 4,
    five = 5,
    six = 6,
    seven = 7,
    eight = 8,
    nine = 9
}
-- local parser = numparser.ParseNums:new()
local parser = numparser.ParseWordNums:new(wordNumMap)
local lineFeed = calibration.LineFeed:new(parser)
local sumLines = calibration.SumLines:new(lineFeed)

local answer = sumLines:process("input.txt")
print(answer)