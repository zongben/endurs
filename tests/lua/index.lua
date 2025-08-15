local d = endurs.driver
local t = endurs.assert
local r = endurs.test_runner

r:describe("First Test", function (test)
  test("test 1", function ()
    print("this is test 1")
  end)
end)


d:close()
