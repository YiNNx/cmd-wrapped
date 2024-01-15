<p align="right">
	English / <a href="./README-CN.md">简体中文</a>
</p>
<div align="center">
    <h1>cmd-wrapped</h1>
    <p>Find out what the past year looks like in commandline!<br/><p/>
	<div>
        <img alt="Stars" src="https://img.shields.io/github/stars/YiNNx/cmd-wrapped?style=flat-square&color=87e3dd&labelColor=444B5A">
        &nbsp;
      	<img alt="Release" src="https://img.shields.io/github/v/release/YiNNx/cmd-wrapped?style=flat-square&color=87e3dd&labelColor=444B5A">
    </div>
    <img src="./assets/image-20240105171950987.png" width="80%" />
</div>



## Features

- Analyze the commandline activity distribution for months, weekdays and hours, as well as frequently-used commands over the past years.
- Github-style command distribution graph
- Supports Zsh, Bash, Fish and Atuin
- A cute Ferris on the cover  <img style="width:25px;vertical-align: bottom;" src="./assets/ferris_hello.gif" />

## How to Use

1. Build from source (recommended, but requires [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed):

   ```shell
   git clone git@github.com:YiNNx/cmd-wrapped.git
   cd cmd-wrapped
   
   # for the past year
   cargo run
   # or for any specific year
   cargo run -- 2022
   # specifying the shell [zsh/bash/fish] or shell history tool [atuin]
   cargo run -- -s zsh
   ```

2. Download from [Release](https://github.com/YiNNx/cmd-wrapped/releases/latest), unzip and run the binary file in the terminal:

   ```shell
   # for the past year
   ./cmd-wrapped
   # or for any specific year
   ./cmd-wrapped 2022
   ```

> [!NOTE]
>
> In some cases, cmd-wrapped may fail to output correct data (such as [all outputs being 0](https://github.com/YiNNx/cmd-wrapped/issues/3)). This is because it relies on Zsh / Bash tracking the timestamp for each command, which requires configuring specific options extraly:
>
> - For Zsh - [EXTENDED_HISTORY](https://zsh.sourceforge.io/Doc/Release/Options.html#History) (oh-my-zsh has it enabled by default)
> - For Bash - [HISTTIMEFORMAT](https://www.gnu.org/software/bash/manual/bash.html#index-HISTTIMEFORMAT)
>
> Commands executed before configuring the option won't be recorded with a timestamp and this will affect cmd-wrapped’s stats. If you find all your stats showing as 0, consider configuring the option now to view the cmd-wrapped stats next year :P

## Credits & License

- Special thanks to [@jyi2ya](https://github.com/jyi2ya) for the cooool idea!
- License: [MIT](https://github.com/YiNNx/cmd-wrapped/blob/master/License)