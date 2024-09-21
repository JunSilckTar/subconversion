# SubConversion

- 本地后端 [tindy2013/subconverter](https://github.com/tindy2013/subconverter), 配置自动生成。

## Table of Contents

- [Requirements](#Requirements)
- [Install](#install)
- [Usage](#usage)
- [Package](#package)
- [License](#license)


## Requirements

需要安装 [node](https://nodejs.org/zh-cn/) 与 [npm](https://www.npmjs.com) 。你可以通过以下命令查看是否安装成功。
- 注：以下步骤为 MacOS 下相应命令，其他系统请自行修改。

```shell
node -v
v18.12.1

npm -v
9.9.3
```

## Install
安装依赖
```shell
npm install
```

## Usage
运行
```shell
npm run dev
```

浏览器访问 <http://localhost:3000/>


## Package
需要安装rust、本机使用的 rustup
```shell
rustup --version
rustup 1.27.1


rustc --version
rustc 1.81.0

# 如果出现问题, 可使用
rustup default stable
```

打包

- 注：在打包之前, 你需要下载对应的后端文件到  <工程目录>/backend/  
  具体可查看 tauri.conf.json 的另外三个系统架构的配置文件
```shell
npm run tauri build
```

## License

Mozilla Public License 2.0 (MPL 2.0) [LICENSE]

© 09-06-2024

JunSilckTar
