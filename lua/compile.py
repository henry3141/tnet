import sys 


class File:
    def __init__(self,name:str) -> None:
        self.name = name
        self.file = open(name,"r").read().split("\n")
        self.includes:list[File] = []
        
    def get_includes(self,path=[]) -> None:
        path.append(self.name)
        for i in self.file:
            if i.startswith("#include"):
                name = i.split(' ')[1]
                if name in path:
                    print(f"Recursion detected: {self.name} -> {name}")
                    sys.exit(1)
                f = File(name)
                f.get_includes(path)
                self.includes.append(f)
                
    def compile(self) -> str:
        self.get_includes()
        l = self.include_list()
        nl = []
        for i in l:
            if not i.name in [i.name for i in nl]:
                nl.append(i)
        l = nl
        return "\n".join([i.to_str() for i in l])
        
        
    def include_list(self) -> list:
        l = []
        for i in self.includes:
            l.append(i)
            l.extend(i.include_list())
        l.append(self)
        return l
        
    
    def to_str(self) -> str:
        code = ""
        for i in self.file:
            if i.startswith("#include"):
                continue
            code += i + "\n"
        return code
        
        
if __name__ == "__main__":
    f = File(sys.argv[1])
    open(sys.argv[2],"w").write(f.compile())