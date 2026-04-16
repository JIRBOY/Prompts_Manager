# 项目：AI 提示词管理器（Prompt.exe）

## 目录结构
```
大模型提示词/
├── source/           # 源代码
│   ├── Cargo.toml    # Rust 项目配置 + 依赖
│   ├── build.rs      # 编译脚本（嵌入 Windows 资源/图标）
│   ├── prompt.rc     # Windows 资源文件（图标 + 版本信息）
│   ├── icons/        # 多尺寸 .ico 图标
│   └── src/
│       └── main.rs   # 主程序（Rust + egui）
├── Prompt.pyw        # 原始 Python/Tkinter 版本（参考）
├── Prompts.json      # 提示词数据文件（运行时读写）
├── other/            # 参考资料/提示词模板
└── dist/             # 编译产物（不纳入 git）
```

## 技术栈
- **Rust** + **egui** (0.31) + **eframe** (glow 后端)
- 图标嵌入：`build.rs` + `rc.exe`（Windows Kit）
- 编译优化：`opt-level = "z"`, `lto = true`, `strip = true`

## 构建
```bash
cd source
cargo build --release
# 输出：source/target/release/prompt.exe → 3.0 MB
```

## 功能
- 搜索 / 标签筛选
- 新增 / 编辑 / 删除提示词
- 复制内容到剪贴板
- 导出到 JSON
- 预览区实时编辑（Ctrl+S 保存）
- 快捷键：Ctrl+N 新增, F2 编辑, Del 删除

## 注意事项
- 数据文件 `Prompts.json` 与 exe 放在同级目录
- 强制浅色主题（`egui::Visuals::light()`），保证控件风格一致
- 中文字体通过加载系统 `msyh.ttc`（微软雅黑）解决
- 窗口启动时通过 `GetSystemMetrics` API 自动屏幕居中
