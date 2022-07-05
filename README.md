# RUST_RSA2_SIGN_WASM Demo

## Usage

### Install

```bash
brew install brotli
brew install llvm
export LLVM_PATH="your llvm path, like /usr/local/opt/llvm"
```

### Build for wechat app

```bash
make build-wx-wasm
```

### Example with Wechat Mini-Program

- 将 pkg 下的 `rsa2_sign.js`、`EncoderDecoderTogether.min.js` 复制到小程序项目中的**同一目录下**.
- 将 pkg 下的 `rsa2_sign_bg.wasm` 或 `rsa2_sign_bg.wasm.br` 复制到小程序项目中。

```javascript
import init, { rsa2_sign } from './rsa2_sign.js'

await init('wasm文件相对小程序项目的位置，如/utils/rsa2_sign_bg.wasm.br')
const res = rsa2_sign('okok')
```

## Refs

1. 找一个简单方便实用的 rsa 库 <https://docs.rs/ring/latest/ring/>
2. ring 这个库 build --target web 会产生一段 `import * as __wbg_star0 from 'env';` <https://github.com/briansmith/ring/issues/1483#issuecomment-1145159978>
3. 引入到小程序时需要注意的坑 <https://juejin.cn/post/7041161141162082340#heading-6>
4. 小程序 ioscacheUnit8Memory0 和 wasm.memory.buffer 不能同步 <https://developers.weixin.qq.com/community/develop/doc/000844fe3a43585c3c2e467795d000>
