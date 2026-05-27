# 按键检测

[English Version](README.md)

一个 Rust TUI 程序，在终端中可视化键盘输入。

## 功能

- 实时键盘可视化
- QWERTY 键盘布局显示
- 按键高亮显示（绿色背景）
- 支持特殊按键
- 简洁的终端界面

## 安装

```bash
git clone https://github.com/yourusername/key-check.git
cd key-check
cargo build --release
```

## 使用

```bash
cargo run
```

按下任意键可在键盘布局上看到高亮显示。
按下 ESC 或 Ctrl+C 退出程序。

## 开发

```bash
# 运行测试
cargo test

# 运行代码检查
cargo clippy

# 格式化代码
cargo fmt
```

## 许可证

MIT 许可证 - 详情见 [LICENSE](LICENSE)。
