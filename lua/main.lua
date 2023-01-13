function class(name,data) -- Create a new class 
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
        local data = self.data[self.index]
        self.data[self.index] = nil
        self.index = self.index - 1
        return data
    end,
    get = function(self,index)
        return self.data[index]
    end,
    set = function(self,index,data)
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
    events = array.new(),
    current_command = nil,
    update = function(self) 
    end,
    execute = function(self)
    end,
    run = function(self)
        while true do
            self:update(self)
        end
    end,
})
main = main.new()
main:run()