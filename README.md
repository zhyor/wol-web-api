# wol-web-api

## cmd

```
wol 6C-1F-F7-0C-A1-4F 

wol 6C:1F:F7:0C:A1:4F 
```
or

```
web-api
```


```
curl http://192.168.1.2:56767/wol?mac=6C-1F-F7-0C-A1-4F 

curl http://192.168.1.2:56767/wol?mac=6C:1F:F7:0C:A1:4F 
```

## docker

```
sudo docker run -d --network=host --name wol-web-api  zhyor/wol-web-api:latest
```
---
docker-compose.yml

```
services:
  wol-web-api:
    image: zhyor/wol-web-api:latest
    container_name: wol-web-api
    network_mode: host
    restart: unless-stopped

```

```
curl http://192.168.1.2:56767/wol?mac=6C-1F-F7-0C-A1-4F 

curl http://192.168.1.2:56767/wol?mac=6C:1F:F7:0C:A1:4F 
```