<p align="right">
	<a href="./README.md">English</a> / 简体中文
</p>
<div align="center">
    <h1>cmd-wrapped</h1>
    <p>👩‍💻 Rust-Based Unix Shell History Analyzer<br/><p/>
	<div>
        <img alt="Stars" src="https://img.shields.io/github/stars/YiNNx/cmd-wrapped?style=flat-square&color=87e3dd&labelColor=444B5A">
        &nbsp;
      	<img alt="Release" src="https://img.shields.io/github/v/release/YiNNx/cmd-wrapped?style=flat-square&color=87e3dd&labelColor=444B5A">
    </div>
</div>


## Features

- 生成过去一年中的命令行活跃分布，如每日最活跃时段，以及常用命令统计。
- 类 Github 的年度命令分布图
- 支持：
  - zsh
  - bash
  - fish
  - atuin

## 如何使用

### 安装

- #### 使用 [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) 安装

```shell
cargo install cmd-wrapped
```

- #### 从 [Release](https://github.com/YiNNx/cmd-wrapped/releases/latest) 下载

下载对应包并解压，在命令行中运行（可能需要 chmod）

- #### Nix❄️

```nix
nix run nixpkgs#cmd-wrapped
```

- #### Arch

```shell
yay -S cmd-wrapped
```

### 参数

```shell
# fetch current stats
cmd-wrapped

# fetch annual shell history stats for a specific year
cmd-wrapped 2024

# specify the target shell
cmd-wrapped -s <shell>
```

所支持的 `<shell>` 选项：`zsh`, `bash`, `fish`, `atuin`. 

> [!NOTE]
>
> 对于某些命令行环境，可能无法输出正确的数据 (比如 [所有数据都为 0](https://github.com/YiNNx/cmd-wrapped/issues/3) ). 这是因为 Zsh / Bash 只有配置了对应的选项后，才会记录下你每一条命令对应的时间：
>
> - Zsh - [EXTENDED_HISTORY](https://zsh.sourceforge.io/Doc/Release/Options.html#History) (oh-my-zsh 则会默认开启此选项)
> - Bash - [HISTTIMEFORMAT](https://www.gnu.org/software/bash/manual/bash.html#index-HISTTIMEFORMAT)
>
> 在没有配置选项前的命令，对应的命令时间不会被持久化记录，因此 cmd-wrapped 也无法获取并生成对应的数据分析。如果运行 cmd-wrapped 发现无法输出正确的数据，也许只能现在配置好选项等待明年来用了 :P

## 致谢 & 开源协议

- 感谢 [@jyi2ya](https://github.com/jyi2ya) 的绝妙想法！
- 开源协议: [MIT](https://github.com/YiNNx/cmd-wrapped/blob/master/LICENSE)
