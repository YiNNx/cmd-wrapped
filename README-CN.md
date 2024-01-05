<p align="right">
	English / <a href="./README-CN.md">简体中文</a>
</p>
<div align="center">
    <h1>cmd-wrapped</h1>
    <p>在命令行中查看你的过去一年！<br/><p/>
	<div>
        <img alt="Stars" src="https://img.shields.io/github/stars/YiNNx/cmd-wrapped?style=flat-square&color=87e3dd&labelColor=444B5A">
        &nbsp;
      	<img alt="Release" src="https://img.shields.io/github/v/release/YiNNx/cmd-wrapped?style=flat-square&color=87e3dd&labelColor=444B5A">
    </div>
    <img src="./assets/image-20240105171950987.png" width="80%" />
</div>


## Features

- 生成过去一年中的命令行活跃分布，如每日最活跃时段，以及常用命令统计。
- 类 Github 的年度命令分布图
- 支持 Zsh & Bash，可通过参数显示过去任意一年的数据。
- 封面的可爱 Ferris<img style="width:25px;vertical-align: bottom;" src="./assets/ferris_hello.gif" />

## 如何运行

1. 从源码构建 (推荐，但需要安装 `cargo`)

   ```shell
   git clone git@github.com:YiNNx/cmd-wrapped.git
   cd cmd-wrapped
   # 查看过去一年
   cargo run
   # 查看指定年份
   cargo run -- 2022
   ```

2. 从 [Release](https://github.com/YiNNx/cmd-wrapped/releases/latest) 中下载对应包并解压，在命令行中运行:

   ```shell
   # 查看过去一年
   ./cmd-wrapped
   # 查看指定年份
   ./cmd-wrapped 2022
   ```

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
- 开源协议: [MIT](https://github.com/YiNNx/cmd-wrapped/blob/master/License)