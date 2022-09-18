# Gna
Gna is a tool for sharing clipboards on multiple computers and supports Mac, Linux and Windows.


# How to use

运行在linux上需要x11环境，建议先按如下命令安装x11环境

```
sudo apt-get install xorg-dev
sudo apt install libxcb-shape0-dev libxcb-xfixes0-dev
```

server模式：

```
gna -p port
```

例如：gna -p 8888 会让gna作为server运行在本机的8888端口

client模式：

```
gna -c ip port
```

例如：gna -c 192.168.10.2 8888 会让gna作为client连接运行在192.168.10.2的8888端口的server

last模式：

```
gna
```

gna每次运行时会在本地缓存命令，如果直接输入gna命令，就会从本地缓存中读取上一次的命令


