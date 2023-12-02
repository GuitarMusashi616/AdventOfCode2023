local numparser = require "numparser"

local function testParsingNums()
    local line = "1and23and4also5bb"

    local parser = numparser.ParseNums:new()

    for number in parser:parse(line) do
        print(number)
    end
end

testParsingNums()