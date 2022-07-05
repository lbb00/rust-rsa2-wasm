.PHONY: build-wasm
build-wasm:
	RUSTC_WRAPPER="" \
	\
	CC_wasm32_unknown_unknown=$(LLVM_PATH)/bin/clang \
	CXX_wasm32_unknown_unknown=$(LLVM_PATH)/bin/clang++ \
	AS_wasm32_unknown_unknown=$(LLVM_PATH)/bin/llvm-as \
	AR_wasm32_unknown_unknown=$(LLVM_PATH)/bin/llvm-ar \
	STRIP_wasm32_unknown_unknown=$(LLVM_PATH)/bin/llvm-strip \
	\
	wasm-pack build \
		--out-dir pkg \
		--target web \
		--release
.PHONY: brotli-wasm
brotli-wasm:
	brotli -f pkg/rsa2_sign_bg.wasm

.PHONY: copy-helper
	cp -rf helper/EncoderDecoderTogether.min.js pkg

.PHONY: build-wx-wasm copy-helper
build-wx-wasm: build-wasm
	patch -p0 pkg/rsa2_sign.js helper/wx.patch

