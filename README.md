# file-patcher

以服务端为基准，下载同步本地文件

用于:

- MC 客户端 MOD 更新和替换
- 可将客户端打包配置好分发给别人

## 服务端功能

- 读取 toml 中所指定的目录
- 遍历该目录下的所有文件，并计算 sha1
- 保存 sha1 到 json 文件中 `file-patcher-server` 目录下
- 提供 API 接口，可获取获取文件，获取元数据

### TOML 设置

`Server.toml` 与主文件同级

```toml
data_path = "./tests/data"
key = "abc"

[[server.files]]
name = "files"
path = "./tests/files"

[[server.files]]
name = "files1"
path = "./tests/files1"
```

`Rocket.toml` 与主文件同级

```toml
[default]
address = "0.0.0.0"
port = 8520
workers = 16
max_blocking = 512
keep_alive = 5
log_level = "normal"
```

### SHA json 文件格式

路径使用相对路径

```json
[
  {
    "name": "a.txt",
    "path": "a.txt",
    "sha1": "abc"
  },
  {
    "name": "b.txt",
    "path": "dir1/b.txt",
    "sha1": "cba"
  }
]
```

### API

- POST: `/update?key=abc` 更新 SHA
- GET: `/list/<name>` 列出相应名称的 SHA 数据
- GET: `<name>/<fils>` 获取文件

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

## 使用方法

### 设置服务端

1. 自行编译或下载 server 二进制文件
2. 双击启动自动生成基本文件
3. 修改 `Server.toml `配置文件，格式见上方
4. 如需修改端口等信息，新建 `Rocket.toml` 并根据上面文档中，修改 `address` `port` 等，具体参考 Rust `Rocket` 配置文档
5. 双击运行，弹出控制台窗口

### 设置客户端

1. 自行编译或下载 client 二进制文件
2. 双击运行，生成配置文件
3. 修改配置文件 `Client.toml`，格式见上方
4. 修改 `to_path` 字段，如为 windows 路径，需把单个 `\` 修改为 `\\` 或 `/`
5. 双击运行
   1. 也可以使用终端进行操作 `client -h`
6. 日志保存在 `output.log` 中，如需 debug, 需要在添加 `-d` 参数后启动
