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