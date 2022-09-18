[中文](#如何使用) [English](#how-to-use)
# Gna
Gna是一个可以在多台电脑之间共享剪切板中文字和图片的工具，使用上类似于苹果设备间的接力，目前支持Linux、Mac和Windows设备之间进行跨平台通信。

Gna is a tool that allows you to share text and images in the clipboard between multiple computers, similar in use to the handoff between Apple devices, and currently supports cross-platform communication between Linux, Mac and Windows devices.

## 如何使用
### 准备工作
1.在linux上运行需要x11环境，建议先按如下命令安装x11环境
```
sudo apt-get install xorg-dev
sudo apt install libxcb-shape0-dev libxcb-xfixes0-dev
```
2.从网页右侧的"Releases"中下载对应平台的压缩包并解压得到二进制文件"gna"。以Mac平台为例，可以执行以下命令方便后续使用
```
chmod +x ~/Downloads/mac-v1.1.1/gna
mv ~/Downloads/mac-v1.1.1/gna /usr/local/bin/gna
```
完成以上设置后，在命令行中输入gna就可以直接执行程序，如果在Mac上第一次运行gna被拦截，需要去"系统偏好设置->安全性与隐私"中允许gna执行

3.如果在linux上以server模式运行gna，可以考虑按如下方法配置开机自启动
```
touch startup_gna.sh
touch /lib/systemd/system/startup_gna.service
```
startup_gna.sh的内容如下
```
#!/usr/bin/env bash
# Filename: startup_gna.sh
cd ~
gna -p 8888
```
startup_gna.service的内容如下
```
[Unit]
Description=Startup Gna
After=network.target
Wants=network.target

[Service]
#需要替换成你创建的startup_gna.sh的绝对路径
ExecStart=/.../startup_gna.sh
ExecStop=/bin/kill $MAINPID
Restart=3

[Install]
WantedBy=multi-user.target
```
编辑完startup_gna.sh和startup_gna.service后，再执行如下命令即可完成开机自启动设置
```
chmod +x startup_gna.sh
systemctl enable startup_gna.service --now
```

### 命令参数

<img width="648" alt="image" src="https://user-images.githubusercontent.com/34652804/190914474-22b0bd28-194e-4ac7-8968-c2453a9d3b73.png">

例子：
```
// 会让gna以server模式运行在本机的8888端口
gna -p 8888

// 会让gna以client模式连接运行在192.168.10.2的8888端口的server
gna -c 192.168.10.2:8888

// 会让gna直接以上次运行的命令再次运行
gna
```


## How to use
### Preparation
1.You need x11 environment to run on linux, it is recommended to install x11 environment first according to the following command
```
sudo apt-get install xorg-dev
sudo apt install libxcb-shape0-dev libxcb-xfixes0-dev
```
2.Download the zip file for your platform from "Releases" on the right side of the page and unzip it to get the binary file "gna". For Mac platform, for example, you can execute the following commands for further use
```
chmod +x ~/Downloads/mac-v1.1.1/gna
mv ~/Downloads/mac-v1.1.1/gna /usr/local/bin/gna
```
After completing the above settings, you can execute the program directly by typing gna in the command line. If gna is intercepted the first time you run it on Mac, you need to go to "System Preferences->Security and Privacy" to allow gna to execute.

3.If you are running gna in server mode on linux, you can consider configuring the boot-up as follows
```
touch startup_gna.sh
touch /lib/systemd/system/startup_gna.service
```
The contents of startup_gna.sh are as follows
```
#!/usr/bin/env bash
# Filename: startup_gna.sh
cd ~
gna -p 8888
```
The contents of startup_gna.service are as follows
```
[Unit]
Description=Startup Gna
After=network.target
Wants=network.target

[Service]
# need to replace with the absolute path of the startup_gna.sh you created
ExecStart=/.../startup_gna.sh
ExecStop=/bin/kill $MAINPID
Restart=3

[Install]
WantedBy=multi-user.target
```
After editing startup_gna.sh and startup_gna.service, execute the following commands to complete the boot-up setup
```
chmod +x startup_gna.sh
systemctl enable startup_gna.service --now
```

### Command parameters

<img width="648" alt="image" src="https://user-images.githubusercontent.com/34652804/190914474-22b0bd28-194e-4ac7-8968-c2453a9d3b73.png">

Example:
```
// will make gna run in server mode on port 8888 of the local machine
gna -p 8888

// will let gna connect to the server running on port 8888 of 192.168.10.2 in client mode
gna -c 192.168.10.2:8888

// will make gna run the command directly again from the previous run
gna
```



