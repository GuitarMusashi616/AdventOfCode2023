-- Part 1 solution

local calibration = require "calibration"
local numparser = require "numparser"

local parser = numparser.ParseNums:new()
local lineFeed = calibration.LineFeed:new(parser)
local sumLines = calibration.SumLines:new(lineFeed)

local answer = sumLines:process("input.txt")
print(answer)