# cmd-wrapped

<p>
	<img alt="Stars" src="https://img.shields.io/github/stars/YiNNx/cmd-wrapped?style=flat-square&color=68BDB7&labelColor=444B5A">
	&nbsp;
	<img alt="Release" src="https://img.shields.io/github/v/release/YiNNx/cmd-wrapped?style=flat-square&color=93AF63&labelColor=444B5A">
	&nbsp;
	<img alt="Release" src="https://img.shields.io/crates/v/cmd-wrapped.svg?style=flat-square&color=C5AB81&labelColor=444B5A">
	&nbsp;
</p>

A CLI to view your shell history stats, with support for zsh, bash, fish, and atuin.

<img src="https://github.com/user-attachments/assets/fa34598f-3b8c-4f90-8569-7724df787b1c" height="750" />

## Installation

- **Use [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)**

```shell
cargo install cmd-wrapped
```

- **Download from [Release](https://github.com/YiNNx/cmd-wrapped/releases/latest)**

Unzip, chmod and run the binary file in the terminal.

- **Archlinux**

```shell
yay -S cmd-wrapped
```

- **Nix❄️**

```nix
nix run nixpkgs#cmd-wrapped
```

## Usage

```sh
# fetch current stats
cmd-wrapped

# fetch annual shell history stats for a specific year
cmd-wrapped 2024

# specify the target shell
cmd-wrapped -s <shell>
```

Supported options for `<shell>` : `zsh`, `bash`, `fish`, `atuin`. 

> [!NOTE]
>
> In some cases, cmd-wrapped may fail to output correct data (such as [all outputs being 0](https://github.com/YiNNx/cmd-wrapped/issues/3)). This is because it relies on the timestamp track for each command, which sometimes requires configuring specific options extraly:
>
> - For Zsh - [EXTENDED_HISTORY](https://zsh.sourceforge.io/Doc/Release/Options.html#History) (oh-my-zsh has it enabled by default)
> - For Bash - [HISTTIMEFORMAT](https://www.gnu.org/software/bash/manual/bash.html#index-HISTTIMEFORMAT)
>
> **Commands executed before configuring the option won't be recorded with a timestamp and this will affect cmd-wrapped’s stats**.

## Credits & License

- Special thanks to [@jyi2ya](https://github.com/jyi2ya) for the cooool idea!
- License: [MIT](https://github.com/YiNNx/cmd-wrapped/blob/master/LICENSE)