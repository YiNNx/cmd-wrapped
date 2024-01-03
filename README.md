<p align="right">
	English / 
	<a href="./README-CN.md">
     简体中文
	</a>
</p>

# cmd-wrapped

Find out what the past year looks like in command-line!

<img src="./assets/image-20240102210902286.png" width="80%" />
<img src="./assets/image-20240102211015987.png" width="80%" />
<img src="./assets/image-20240102211113761.png" width="80%" />

## Features

- Support Zsh & Bash
- Get your cmd wrapped for any specific year with argument

## How to Use

1. Build from source (recommended, but requires `cargo` installed):

   ```shell
   git clone git@github.com:YiNNx/cmd-wrapped.git
   cd cmd-wrapped
   # for the past year
   cargo run
   # or for any specific year
   cargo run -- 2022
   ```

2. Download from [Release](https://github.com/YiNNx/cmd-wrapped/releases/latest), unzip and run the binary file in the terminal:

   ```shell
   # for the past year
   ./cmd-wrapped
   # or for any specific year
   ./cmd-wrapped 2022
   ```

## Credits & License

- Special thanks to [@jyi2ya](https://github.com/jyi2ya) for the cooool idea :kissing_heart:
- License: [MIT](https://github.com/YiNNx/cmd-wrapped/blob/master/License)