
local numparser = require "numparser"


local function testWordNumParser()
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
    -- local line = "fivepqxlpninevh2xxsnsgg63pbvdnqptmg"
    local line = "eightwothree18nineight"
    local parser = numparser.ParseWordNums:new(wordNumMap)
    local numbers = {}

    for number in parser:parse(line) do
        numbers[#numbers+1] = number
        print(number)
    end
end

testWordNumParser()


