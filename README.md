# easycom

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

一个用于读取串口数据的小工具，简单易用是宗旨(因为复杂的不会写)

以后的更新应该是着重于命令行交互的体验优化方面吧(翻译:多的咱也不会了)。
如果有可能，我会把这个做成真正强大的串口调试工具，类似与picocom，或者重新开一个项目....谁知道呢。

菜的安详 (我名字都叫easycom了，怎么可能强大起来)

## 内容列表

- [背景](#背景)
- [安装](#安装)
- [使用说明](#使用说明)
- [维护者](#维护者)
- [如何贡献](#如何贡献)

## 背景

刚学rust不久，想着造点轮子也好，代码很烂，写的不好的地方希望多骂骂我....

## 安装

```sh
$ git clone https://github.com/YEOLLL/easycom
$ cargo build --release
$ ./target/release/easycom
```

或者

```sh
$ git clone https://github.com/YEOLLL/easycom
$ cargo run
```

## 使用说明

```sh
$ ./easycom -h
"""
easycom 0.1.0
YEOL
about应该怎么填......

USAGE:
    easycom [OPTIONS] [device]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --baud_rate <baud_rate>          波特率 [default: 9600]
        --data_bits <data_bits>          表示行上发送的字符的位数[5, 6, 7, 8] [default: 8]
        --flow_control <flow_control>    控制数据传输的信令类型[None, Software, Hardware] [default: None]
        --parity <parity>                错误检查的奇偶校验类型[None, Odd, Even] [default: None]
        --stop_bits <stop_bits>          表示字符结尾的位数[1, 2] [default: 1]
        --timeout <timeout>              等待接收数据的超时时间(毫秒) [default: 5]

ARGS:
    <device>    tty端口设备
"""
```

注意：不输入任何参数将没有输出  

 

举个栗子：

```sh
$ ./easycom /dev/ttyUSB0
```

```sh
$ ./easycom -b 115200 /dev/ttyUSB0
```

```sh
$ ./easycom /dev/ttyUSB0 --timeout=10 --parity=Odd
```

## 维护者

[@YEOLLL](https://github.com/YEOLLL) 只有我一个，可怜，无助。

## 如何贡献

我不配
