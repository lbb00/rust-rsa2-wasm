## Usage

### build-wasm

```bash
brew install llvm
LLVM_PATH=/opt/homebrew/opt/llvm/
make build-wasm
```

## lbb 后记

被坑了N次：

1. 找一个简单方便实用的 rsa 库
2. ring 这个库 build --target web 会产生一段 `import * as __wbg_star0 from 'env';` https://github.com/briansmith/ring/issues/1483#issuecomment-1145159978
3. 引入到小程序时需要注意的坑 https://juejin.cn/post/7041161141162082340#heading-6
4. https://developers.weixin.qq.com/community/develop/doc/000844fe3a43585c3c2e467795d000
