
local alias = "a.b/c\\d-e"
local sanitized = alias:gsub("%p", {["/"] = "", ["."] = ""})
print("Original: " .. alias)
print("Sanitized: " .. sanitized)

local null_alias = "test\0test"
local null_sanitized = null_alias:gsub("%p", {["/"] = "", ["."] = ""})
print("Null Original Len: " .. #null_alias)
print("Null Sanitized Len: " .. #null_sanitized)

if sanitized == "abc\\d-e" then
    print("VERIFIED: Only . and / removed. Backslash kept.")
else
    print("FAILED: Backslash removed or other behavior.")
end
