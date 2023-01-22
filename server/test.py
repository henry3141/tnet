import requests 
import time
from threading import Thread
#send a pure string
x = []
for i in range(1000):
    now = time.perf_counter()
    requests.post('http://127.0.0.1:8000/update', data='[{"x":0,"y":0,"z":0,"block":"Air"},{"x":1,"y":1,"z":1,"block":"Air"},{"x":2,"y":2,"z":2,"block":"Air"}]')
    x.append(time.perf_counter()-now)
print("median_time:",sum(x)/len(x))