---@class ITrie
local ITrie = {}

---@param word string
---@param value any
function ITrie:insert(word, value)  --luacheck: ignore
    assert(false, "Method must be implemented")
end

---@param needle string
---@return any?
function ITrie:checkFor(needle)  --luacheck: ignore
    assert(false, "Method must be implemented")
end

return ITrie

