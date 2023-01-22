function print_table(table) 
    for k,v in pairs(table) do
        print(k,v)
    end
end
function class(data) 
    local class = {}
    class.__prototype = data
    class.new = function(data)
        if data == nil then data = {} end
        local instance = class.__prototype
        for k,v in pairs(data) do
            instance[k] = v
        end
        return instance
    end
    return class
end
turtle_class = class({
    position = {x=0,y=0,z=0},
    move = function(self,direction) 
        print_table(direction)
    end,
})

goto_class = class({
    target =  function(self, target)
        self.target = target
    end,
    step = function(self,turtle_)
        turtle_:move(self:best(turtle_))
    end,
    distance = function(pos1,pos2) 
        return math.sqrt((pos1.x-pos2.x)^2+(pos1.y-pos2.y)^2+(pos1.z-pos2.z)^2)
    end,
    is_reached = function(self)
        return self.distance(self.target,self.turtle_.position) < 0.1
    end,
    best = function(self,turtle_)
        --you can go x y or z 1/-1 wich gets you the nearest to the target
        best_distance = 0
        best_move = {x=0,y=0,z=0}
        for i = -1,1 do 
            local new_pos = {x=turtle_.position.x + i,y=turtle_.position.y,z=turtle_.position.z}
            local new_distance = self.distance(self.target,new_pos)
            if new_distance < best_distance then
                best_distance = new_distance
                best_move = {x=i,y=0,z=0}
            end
        end
        for i = -1,1 do 
            local new_pos = {x=turtle_.position.x,y=turtle_.position.y + i,z=turtle_.position.z}
            local new_distance = self.distance(self.target,new_pos)
            if new_distance < best_distance then
                best_distance = new_distance
                best_move = {x=0,y=i,z=0}
            end
        end
        for i = -1,1 do 
            local new_pos = {x=turtle_.position.x,y=turtle_.position.y,z=turtle_.position.z + i}
            local new_distance = self.distance(self.target,new_pos)
            if new_distance < best_distance then
                best_distance = new_distance
                best_move = {x=0,y=0,z=i}
            end
        end
        return best_move
    end,
})
turtle_ = turtle_class.new()
goto_ = goto_class.new()
goto_:target({x=0,y=10,z=0})
print_table(goto_.target)
goto_:step(turtle_)
