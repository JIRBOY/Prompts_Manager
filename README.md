# AI 提示词管理器 (Prompt.exe)

一个轻量、快速的 AI 提示词管理工具，用于收藏、搜索和一键复制常用提示词。

## 功能

- **搜索与筛选** — 按关键词搜索或按标签快速筛选
- **增删改查** — 新增、编辑、删除提示词
- **一键复制** — 点击即可将提示词内容复制到剪贴板
- **实时预览编辑** — 选中后可在右侧预览区查看和编辑内容（Ctrl+S 保存）
- **导出备份** — 将全部提示词导出为 JSON 文件
- **快捷键** — `Ctrl+N` 新增、`F2` 编辑、`Del` 删除

## 特性

| 指标 | 值 |
|------|-----|
| 可执行文件大小 | ~3.0 MB |
| 技术栈 | Rust + egui + eframe (glow) |
| 平台 | Windows 10+ |
| 依赖 | 无（独立运行，无需安装运行时） |

## 快速开始

### 使用已编译版本

将 `Prompt.exe` 和 `Prompts.json` 放在**同一目录**下，双击运行即可。

### 从源码编译

```bash
cd source
cargo build --release
```

编译产物位于 `source/target/release/prompt.exe`，约 3.0 MB。

**前置条件：**

- Rust 工具链（`rustup` 安装）
- 如需嵌入图标：Windows Kit `rc.exe`（可选，不编译 `.rc` 也不影响功能）

## 项目结构

```
大模型提示词/
├── source/           # Rust 源码
│   ├── Cargo.toml    # 依赖配置
│   ├── build.rs      # Windows 资源编译
│   ├── prompt.rc     # 图标与版本信息
│   ├── icons/        # .ico 图标文件
│   └── src/
│       └── main.rs   # 主程序入口
├── Prompt.exe        # 编译后的可执行文件
├── Prompts.json      # 提示词数据（运行时读写）
├── Prompt.pyw        # 原始 Python/Tkinter 版本（参考）
├── other/            # 参考资料/提示词模板
└── dist/             # 旧版 PyInstaller 产物
```

## 数据来源

`Prompts.json` 与 exe 同级目录，首次启动时若不存在会自动创建示例数据。格式：

```json
[
  {
    "id": 1,
    "title": "提示词标题",
    "content": "提示词正文内容...",
    "tags": ["标签1", "标签2"],
    "createTime": "2025-01-01 00:00:00",
    "updateTime": "2025-01-01 00:00:00"
  }
]
```

## 开发说明

- 强制浅色主题，确保控件风格一致
- 中文字体通过加载系统 `msyh.ttc`（微软雅黑）解决
- 窗口启动时自动屏幕居中
- Release 编译优化：`opt-level = "z"`, `lto = true`, `strip = true`, `panic = "abort"`

## License

本项目仅供个人学习与使用。
