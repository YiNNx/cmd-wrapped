# cmd-wrapped

<p>
	<img alt="Stars" src="https://img.shields.io/github/stars/YiNNx/cmd-wrapped?style=flat-square&color=68BDB7&labelColor=444B5A">
	&nbsp;
	<img alt="Release" src="https://img.shields.io/github/v/release/YiNNx/cmd-wrapped?style=flat-square&color=93AF63&labelColor=444B5A">
	&nbsp;
	<img alt="Release" src="https://img.shields.io/crates/v/cmd-wrapped.svg?style=flat-square&color=C5AB81&labelColor=444B5A">
	&nbsp;
</p>

一个用于查看 shell 历史记录统计信息的 CLI，支持 zsh、bash、fish 和 atuin。

<img src="https://github.com/user-attachments/assets/fa34598f-3b8c-4f90-8569-7724df787b1c" height="750" />

## 安装

- **使用 [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)**

```shell
cargo install cmd-wrapped
```

- **从 [Release](https://github.com/YiNNx/cmd-wrapped/releases/latest) 下载**

解压，赋予执行权限，然后在终端中运行二进制文件。

- **Archlinux**

```shell
yay -S cmd-wrapped
```

- **Nix❄️**

```nix
nix run nixpkgs#cmd-wrapped
```

## 使用

```sh
# 获取当前统计数据
cmd-wrapped

# 获取特定年份的年度 shell 历史统计数据
cmd-wrapped 2024

# 指定目标 shell
cmd-wrapped -s <shell>
```

对 `<shell>` 支持的选项：`zsh`、`bash`、`fish`、`atuin`。

> [!NOTE]
>
> 在某些情况下，cmd-wrapped 可能无法输出正确的数据（例如 [所有输出均为 0](https://github.com/YiNNx/cmd-wrapped/issues/3)）。这是因为它依赖于每个命令的时间戳记录，有时需要额外配置特定选项：
>
> - 对于 Zsh - [EXTENDED_HISTORY](https://zsh.sourceforge.io/Doc/Release/Options.html#History)（oh-my-zsh 默认启用）
> - 对于 Bash - [HISTTIMEFORMAT](https://www.gnu.org/software/bash/manual/bash.html#index-HISTTIMEFORMAT)
>
> **在配置选项之前执行的命令将不会记录时间戳，这将影响 cmd-wrapped 的统计数据**。

## 致谢与许可证

- 特别感谢 [@jyi2ya](https://github.com/jyi2ya) 的绝妙想法！
- 许可证：[MIT](https://github.com/YiNNx/cmd-wrapped/blob/master/LICENSE)