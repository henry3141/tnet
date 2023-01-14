import requests
while True:
    print(requests.get('http://127.0.0.1:8000/command/1/1').content)