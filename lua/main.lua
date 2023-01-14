function class(name,data) 
    local class = {}
    class.name = name
    class.prototype = data
    class.new = function(data)
        if data == nil then data = {} end
        local instance = class.prototype
        for k,v in pairs(data) do
            instance[k] = v
        end
        return instance
    end
    return class
end 

array = class("array",{
    data = {},
    index = 1,
    push = function(self,data)
        self.data[self.index] = data
        self.index = self.index + 1
    end,
    pop = function(self)
        if self.index == 1 then return nil end
        local data = self.data[self.index]
        self.data[self.index] = nil
        self.index = self.index - 1
        return data
    end,
    get = function(self,index)
        if index > self.index then return nil end
        return self.data[index]
    end,
    set = function(self,index,data)
        if index > self.index then return nil end
        self.data[index] = data
    end,
    size = function(self)
        return self.index
    end,
    clear = function(self)
        self.data = {}
        self.index = 1
    end,

})


event = class("event",{
    kind="",
    data={},
})

json = class("json",{
    from_string = function(string)
        local data = textutils.unserializeJSON(string)
        return data
    end,
    to_string = function(data)
        local string = textutils.serializeJSON(data)
        return string
    end,
}).new()


goto_ = class("goto",{
    jump_len=1,
    target={x=0,y=0,z=0},
    step = function(self,turtle) 
        if turtle.position.x == self.target.x and turtle.position.y == self.target.y and turtle.position.z == self.target.z then
            turtle.current_command = nil
            return
        end
        local steps = array.new()
        local pos = turtle.position
        local jump_len = self.jump_len
        local best = self:best(pos)
        turtle:move(best)
    end, 
    distance = function(pos1,pos2) 
        local x = pos1.x - pos2.x
        local y = pos1.y - pos2.y
        local z = pos1.z - pos2.z
        return math.sqrt(x*x + y*y + z*z)
    end,
    best = function(self,pos1)
        -- return the best direction to go in form of a vector where only one value is 1 or -1 and you
        local best = {x=0,y=0,z=0}
        local best_distance = self.distance(pos1,self.target)
        for x=-1,1,2 do
            local new_pos = {x=pos1.x+x,y=pos1.y,z=pos1.z}
            local new_distance = self.distance(new_pos,self.target)
            if new_distance < best_distance then
                best_distance = new_distance
                best = {x=x,y=0,z=0}
            end
        end
        for y=-1,1,2 do
            local new_pos = {x=pos1.x,y=pos1.y+y,z=pos1.z}
            local new_distance = self.distance(new_pos,self.target)
            if new_distance < best_distance then
                best_distance = new_distance
                best = {x=0,y=y,z=0}
            end
        end
        for z=-1,1,2 do
            local new_pos = {x=pos1.x,y=pos1.y,z=pos1.z+z}
            local new_distance = self.distance(new_pos,self.target)
            if new_distance < best_distance then
                best_distance = new_distance
                best = {x=0,y=0,z=z}
            end
        end
        return best
    end,
})

local function get_keys(t)
    local keys={}
    for key,_ in pairs(t) do
      table.insert(keys, key)
    end
    return keys
  end


main = class("main",{
    position = {x=0,y=0,z=0},
    current_command = nil,
    set = 0,
    id = 0,
    last_command = nil,
    facing = "x+",
    update = function(self) 

    end,
    execute = function(self)
        if self.current_command ~= nil then
            self.current_command:step(self)
        end
    end,
    connection = function(self)
        local command = json.from_string(http.get("http://127.0.0.1:8000/command/"..self.set.."/"..self.id).readAll())
        if command ~= "Cancel" then
            local keys = get_keys(command)
            if keys[1] == "Goto" then
                local goto__ = goto_.new()
                goto__.target = {x=command.Goto[1],y=command.Goto[2],z=command.Goto[3]}
                self.current_command = goto__
            end
        else 
            self.current_command = nil
        end
    end,
    run = function(self)
        while true do
            self:update()
            self:execute()
            self:connection()
            if self.current_command == nil then
                sleep(2)
            end
        end
    end,
    move = function(self,direction)
        local x = direction.x
        local y = direction.y
        local z = direction.z
        if self.facing == "x-" then
            turtle.turnLeft()
            turtle.turnLeft()
        elseif self.facing == "z+" then
            turtle.turnLeft()
        elseif self.facing == "z-" then
            turtle.turnRight()
        end 
        self.facing = "x+"
        if x == 1 then
            turtle.forward()
            self.facing = "x+"
        elseif x == -1 then
            turtle.back()
            self.facing = "x-"
        elseif y == 1 then
            turtle.up()
        elseif y == -1 then
            turtle.down()
        elseif z == 1 then
            turtle.turnRight()
            turtle.forward()
            self.facing = "z+"
        elseif z == -1 then
            turtle.turnLeft()
            turtle.forward()
            self.facing = "z-"
        end
        self.position.x = self.position.x + x
        self.position.y = self.position.y + y
        self.position.z = self.position.z + z
    end,
}).new()
main:run()