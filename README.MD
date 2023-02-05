# usb1020-rs（未完成）

本项目并没有开发到可用状态！但我已经没有测试的机会了。

根据官方头文件与逆向工程结果，使用 Rust 编写的 USB1020 运动控制卡用户态驱动。

- 无 `MFC80.dll` 运行时依赖 
- 无需关闭驱动强制签名
- 跨平台

## 运行依赖

需要系统中已安装 `libusb-1.0`，Windows 上需要为设备安装 `WinUSB` 等兼容 `libusb` 的通用驱动，具体请参照 `rusb` 的 [README](https://github.com/a1ien/rusb) 与 `libusb` 的 [Wiki](https://github.com/libusb/libusb/wiki#supported-environments)。

## 声明

本仓库与北京阿尔泰科技发展有限公司或其任何子公司或其附属机构没有任何关系、关联、授权、认可，也没有任何正式联系。
