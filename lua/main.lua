#include print_table.lua
#include class.lua
#include turtle.lua
#include goto.lua
turtle_ = turtle_class.new()
goto_ = goto_class.new()
goto_:target({x=0,y=10,z=0})
print_table(goto_.target)
goto_:step(turtle_)