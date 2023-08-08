# file-patcher

## 服务端功能

- 读取toml中所指定的目录
- 遍历该目录下的所有文件，并计算sha1
- 保存sha1到json文件中 `file-patcher-server` 目录下
- 提供API接口，可获取获取文件，获取元数据


### TOML设置
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
        "name":"a.txt",
        "path":"a.txt",
        "sha1":"abc"
    },
    {
        "name":"b.txt",
        "path":"dir1/b.txt",
        "sha1":"cba"
    }
]
```

### API

- POST: `/update?key=abc` 更新SHA
- GET: `/list/<name>` 列出相应名称的SHA数据
- GET: `<name>/<fils>` 获取文件

## 客户端

### TOML 设置

```toml
[client]
host = "127.0.0.1"
port = 1234
key = "abcd"

[[files]]
name = "abc"
to_path = "D:\\Path"

[[files]]
name = "cba"
to_path = "abc\\cba"
```