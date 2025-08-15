local e = endurs

e:goto("https://www.google.com/")
local query_input = e:find("id", "APjFqb")
query_input:send_keys("test")

local btn = e:find("name", "btnK")
btn:click()
e:close()
