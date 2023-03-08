<h1 align="center">
  <img src="./src/assets/image/logo.png" alt="Clash" width="128" />
  <br>
  Hiddify Desktop
  <br>
</h1>

<h3 align="center">
A <a href="https://github.com/MetaCubeX/Clash.Meta">Clash Meta</a> GUI based on <a href="https://github.com/tauri-apps/tauri">tauri</a>.
</h3>

## Why HiddifyDesktop?
- It supports Vless and custom browser fingerprinting (simulate Chrome fingerprint for example)
- Optimized for beginners
- Support deep linking in windows (You can click on clash links and it will import the config)
- Auto select the profile after download
- And more is comming :-)

## Features
- Supports Trojan, Vless, Vmess, Shadowsocks, v2ray, SSR, ShadowTLS, tuic, ....
- Rule based proxy. So you can easily ignore not blocked sites :)
- Full `clash` config supported, Partial `clash premium` config supported.
- Profiles management and enhancement (by yaml and Javascript). [Doc](https://github.com/zzzgydi/clash-verge/wiki/%E4%BD%BF%E7%94%A8%E6%8C%87%E5%8D%97)
- Simple UI and supports custom theme color.
- Built-in support [Clash.Meta](https://github.com/MetaCubeX/Clash.Meta) core.
- System proxy setting and guard.

## Install

Download from [release](https://github.com/hiddify/HiddifyDesktop/releases). Supports Windows x64, Linux x86_64 and macOS 11+

Or you can build it yourself. Supports Windows, Linux and macOS 10.15+

Notes: If you could not start the app on Windows, please check that you have [Webview2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/#download-section) installed.

## Development

You should install Rust and Nodejs, see [here](https://tauri.app/v1/guides/getting-started/prerequisites) for more details. Then install Nodejs packages.

```shell
yarn install
```

Then download the clash binary... Or you can download it from [clash premium release](https://github.com/Dreamacro/clash/releases/tag/premium) and rename it according to [tauri config](https://tauri.studio/docs/api/config/#tauri.bundle.externalBin).

```shell
yarn run check
```

Then run

```shell
yarn dev
```

Or you can build it

```shell
yarn build
```

## Todos

> This keng is a little big...

## Screenshots

<div align="center" markdown="1">

<img width="24%" src="https://user-images.githubusercontent.com/114227601/223690647-5cfbb09f-66a5-4990-b34d-bfadd36108ae.png" />
<img width="24%" src="https://user-images.githubusercontent.com/114227601/223690953-37efa122-c876-40b8-80c9-762e26835080.png" />

<img width="24%" src="https://user-images.githubusercontent.com/114227601/223690817-607926be-2a60-4d53-8fac-e8b1cfff6684.png" />
<img width="24%" src="https://user-images.githubusercontent.com/114227601/223690854-7e8d88bf-15af-438f-8950-6d9804a0fa53.png" />
</div>

## Disclaimer

This is a learning project for Rust practice.

## Contributions

Issue and PR welcome!

## Acknowledgement

Hiddify Desktop was based on or inspired by these projects and so on:

- [clash-verge](https://github.com/zzzgydi/clash-verge): directly forked
- [tauri-apps/tauri](https://github.com/tauri-apps/tauri): Build smaller, faster, and more secure desktop applications with a web frontend.
- [Dreamacro/clash](https://github.com/Dreamacro/clash): A rule-based tunnel in Go.
- [MetaCubeX/Clash.Meta](https://github.com/MetaCubeX/Clash.Meta): A rule-based tunnel in Go.
- [Fndroid/clash_for_windows_pkg](https://github.com/Fndroid/clash_for_windows_pkg): A Windows/macOS GUI based on Clash.
- [vitejs/vite](https://github.com/vitejs/vite): Next generation frontend tooling. It's fast!
- [Tauri Plugin deep-link](https://github.com/FabianLars/tauri-plugin-deep-link): Implements Deep-link(URI-Scheme) support in Tauri

## License

GPL-3.0 License. See [License here](./LICENSE) for details.
