#include print_table.lua
#include class.lua
turtle_class = class({
    position = {x=0,y=0,z=0},
    move = function(self,direction) 
        print_table(direction)
    end,
})