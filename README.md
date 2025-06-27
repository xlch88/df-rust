# df-rust

本项目提供了一个类似 `df --output=fstype,source,size,used,avail,pcent,target -B M` 的功能，用于跨平台获取磁盘信息的 Rust 实现，并通过 NAPI 导出到 JavaScript/TypeScript 环境中。

## 功能特性

-   获取所有磁盘信息：包括文件系统类型、挂载点、总空间、已用空间、可用空间以及使用率等
-   跨平台支持
-   通过 [sysinfo](https://docs.rs/sysinfo/) 库获取磁盘信息，可靠高效

## 安装

在项目根目录构建并安装：

```bash
npm install
npm run rebuild
```

## 使用示例

```javascript
import df from "df-rust";

const disks = df();
console.log(disks);
```

返回结果示例（部分字段展示）：

```json
[
	{
		"fstype": "NTFS",
		"source": "C:\\",
		"size": 100000,
		"used": 50000,
		"avail": 50000,
		"pcent": "50%",
		"target": "C:\\"
	}
]
```

## 开发/构建

```bash
npm install
npm run rebuild
```

## 许可证

使用 MIT 许可证。
