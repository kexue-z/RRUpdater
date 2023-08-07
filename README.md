# file-patcher

## 服务端功能

- 读取toml中所指定的目录
- 遍历该目录下的所有文件，并计算sha1
- 保存sha1到json文件中 `file-patcher-server` 目录下
- 提供API接口，可获取获取文件，获取元数据



### TOML设置
与主文件同级
```toml
[server]
port = 1234
key = "abcd"
files_dir = [
    {name = "abc", dir = "/path"},
    {name = "cba", dir = "./path"}
]
```

### json 文件格式
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