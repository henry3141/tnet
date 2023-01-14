function class(name,data) 
    local class = {}
    class.name = name
    class.prototype = {}
    for i in pairs(data) do
        class.prototype[i] = data[i]
    end
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

function print_table(table) 
    for k,v in pairs(table) do
        print(k,v)
    end
end

array = class("event",{
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



main = class("main",{
    position = {x=0,y=0,z=0},
    current_command = nil,
    set = 0,
    id = 0,
    last_command = nil,
    update = function(self) 

    end,
    execute = function(self)
    end,
    connection = function(self)
        while true do 
            sleep(1)
            local command = http.get("http://127.0.0.1:8000/command/"..self.set.."/"..self.id)
            print(command.readAll())
        end
    end,
    run = function(self)
        self:connection(self)
    end,
})
main = main.new()
main:run()