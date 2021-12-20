# Gna
Gna is a tool to share clipboard over multi computers

Gna 是一个可以在多台电脑间共享剪切板的工具，支持mac、linux、windows。

请注意gna在传输数据时没有使用任何加密措施（建议仅在局域网使用），不要用其在公共网络传输私密数据！！！

名称来源：盖娜，Gna、Gná。在北欧神话中，是“风”的化身，奥丁妻子弗丽嘉的仕女之一，主要的工作是作为弗丽嘉的信使。

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


