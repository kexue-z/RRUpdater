# rr_updater

以服务端为基准，下载同步本地文件

用于:

- MC 客户端 MOD 更新和替换
- 可将客户端打包配置好分发给别人
- 其他需要以服务端为准的同步文件需求

## 使用方法

### 设置服务端

1. 自行编译或下载 `rrus` 二进制文件
2. 双击启动自动生成基本文件
3. 新建 `Rocket.toml` 并根据上面文档中，修改 `address` `port` 等，具体参考 Rust `Rocket` 配置文档
4. 添加 `[[default.rr_config]]` 配置项目，填写`name` `path` 等信息, 并修改 `key` `data_path` 
   1. `path` 为同步文件所在路径 可以为相对路径
   2. `data_path` 为缓存json存储路径 建议为 `./data`
5. 双击运行，弹出控制台窗口

### 设置客户端

**初次配置:**

1. 自行编译或下载 `rruc` 二进制文件
2. 双击运行，生成配置文件
3. 修改配置文件 `Client.toml`，格式见下方
4. 修改 `to_path` 字段，如为 windows 路径，需把单个 `\` 修改为 `\\` 或 `/`
5. 双击运行
   1. 也可以使用终端进行操作 `rruc -h`
6. 日志保存在 `output.log` 中，如需 debug, 需要在添加 `-d` 参数后启动

**分发给别人:**

1. 打包 `rruc` 和 `Client.toml` 两个文件，并分发
2. 如果目录结构确定，如 `.minecarft/mods` 则不需要要求修改 `to_path`
3. 如果路径有所不同，需要手动编辑 `Client.toml` 更改为相应的路径

## 服务端功能

- 读取 toml 中所指定的目录
- 遍历该目录下的所有文件，并计算 sha1
- 保存 sha1 到 json 文件中 data_path 目录下
- 提供 API 接口，可获取获取文件，获取元数据

### TOML 设置

`Rocket.toml` 与主文件同级

```toml
[default]
address = "0.0.0.0"
port = 8520
workers = 16
max_blocking = 512
keep_alive = 5
log_level = "normal"

key = "abc"
data_path = "./tests/data"

[[default.rr_config]]
name = "files"
path = "./tests/files"

[[default.rr_config]]
name = "files1"
path = "./tests/files1"
```

### API

- POST: `/update?key=abc` 更新 SHA
- GET: `/list/<name>` 列出相应名称的 SHA 数据
- GET: `/<name>/<fils>` 获取文件

## 客户端

### TOML 设置

```toml
[client]
host = "http://127.0.0.1:8520"
key = "abc"
data_path = "./tests/client_data"

[[sync]]
name = "files"
to_path = "./tests/files"

[[sync]]
name = "files1"
to_path = "./tests/files1"
```

