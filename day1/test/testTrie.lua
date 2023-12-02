local common = require "common"

local function testTrie()
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
    local trie = common.Trie:fromMap(wordNumMap)
    local exists = trie:checkFor("fiv")
    assert(exists ~= nil)
    -- print(exists)
    local five = trie:checkFor("five")
    assert(five == 5)
    -- print(five)
    local noexist = trie:checkFor("iv")
    assert(noexist == nil)
    -- print(noexist)
end

testTrie()