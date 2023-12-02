---@class Iterator
local Iterator = {}

---Returns a value when called until it no longer returns a value marking the end of the iteration
---@return any?
function Iterator:__call()  --luacheck: ignore
    assert(false, "Method must be implemented")
end

return Iterator