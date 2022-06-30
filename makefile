build-wasm:
	RUSTC_WRAPPER="" \
	\
	CC_wasm32_unknown_unknown=${LLVM_PATH:?}/bin/clang \
	CXX_wasm32_unknown_unknown=${LLVM_PATH:?}/bin/clang++ \
	AS_wasm32_unknown_unknown=${LLVM_PATH:?}/bin/llvm-as \
	AR_wasm32_unknown_unknown=${LLVM_PATH:?}/bin/llvm-ar \
	STRIP_wasm32_unknown_unknown=${LLVM_PATH:?}/bin/llvm-strip \
	\
	wasm-pack build \
		--out-dir "${output_path:?}" \
		--target web \
		--release
