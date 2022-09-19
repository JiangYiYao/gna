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



