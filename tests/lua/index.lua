local d = endurs.driver

d:goto("https://www.google.com/")
local query_input = d:find("id", "APjFqb")
query_input:send_keys("test")

local btn = d:find("name", "btnK")
btn:click()
d:close()
