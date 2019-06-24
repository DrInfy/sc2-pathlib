import sc2pathlib
array=[[1,1,1,1],[1,1,0,1],[1,1,0,1]]
start =(0,0)
end=(2,0)
print(sc2pathlib.PathFind(array).find_path(start,end))

