# Gna
Gna是一个在多台电脑上共享剪贴板的工具，支持Mac、Linux和Windows。

Gna is a tool for sharing clipboards on multiple computers and supports Mac, Linux and Windows.


# How to use

运行在linux上需要x11环境，建议先按如下命令安装x11环境

You need x11 environment to run on linux, it is recommended to install x11 environment first according to the following command

```
sudo apt-get install xorg-dev
sudo apt install libxcb-shape0-dev libxcb-xfixes0-dev
```

从release中下载对应平台的二进制文件"gna"，然后执行以下命令给gna可执行权限，并放入/usr/local/bin目录

Download the platform-specific binary "gna" from the release, then execute the following command to give gna executable permissions and place it in the /usr/local/bin directory

```
chmod 777 ~/Downloads/gna
mv ~/Downloads/gna /usr/local/bin/gna
```

完成以上设置后，在命令行中输入gna就可以直接执行程序，如果在mac系统上第一次运行gna被拦截，需要去"系统偏好设置->安全性与隐私"中允许gna执行，gna的使用命令有如下3种：

After completing the above settings, you can directly execute the program by typing gna in the command line. If gna is intercepted the first time you run it on a mac system, you need to go to "System Preferences->Security and Privacy" to allow gna to execute. There are 3 commands for using gna as follows:

```
gna
gna -p port
gna -c server_ip:server_port
```
使用例子：

Example of use：

```
gna -p 8888
```
会让gna以server模式运行在本机的8888端口

will let gna run in server mode on port 8888 of the local machine


```
gna -c 192.168.10.2:8888
```
会让gna以client模式连接运行在192.168.10.2的8888端口的server

will let gna connect to the server running on port 8888 of 192.168.10.2 in client mode


```
gna
```
会让gna直接以上次运行的命令再次运行

will let gna run the command directly from the previous run again

