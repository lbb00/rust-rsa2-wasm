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
copy-helper:
	cp -rf helper/EncoderDecoderTogether.min.js pkg/

.PHONY: delete-dist
delete-dist:
	rm -rf pkg

.PHONY: build-wx-wasm
build-wx-wasm: delete-dist build-wasm copy-helper brotli-wasm
	cp pkg/rsa2_sign.js pkg/rsa2_sign.wx.js && \
	patch -p0 pkg/rsa2_sign.wx.js helper/wx.patch
