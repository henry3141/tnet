import sys 

def get_includes(file,includes:list[str]) -> list[str]:
    file = open(file,"r").read().split("\n")
    for i in file:
        if i.startswith("#include"):
            file_name = i.split(" ")[1]
            if file_name not in includes:
                includes.append(file_name)
                includes = get_includes(file_name,includes)
    return includes
            
    

file = open(sys.argv[1],"r").read()
includes = get_includes(sys.argv[1],[])
file_two = ""
for i in includes:
    file_two += open(i,"r").read() + "\n"
file_two += file
file = file_two.split("\n")
file_two = ""
for i in file:
    if not i.startswith("#include"):
        file_two += i + "\n"
open("o.lua","w").write(file_two)
