SHELL := /bin/bash

debug:
	EMCC_CFLAGS="-O3 -sUSE_GLFW=3 -sGL_ENABLE_GET_PROC_ADDRESS -sWASM=1 -sALLOW_MEMORY_GROWTH=1 -sWASM_MEM_MAX=512MB -sTOTAL_MEMORY=512MB -sABORTING_MALLOC=0 -sASYNCIFY -sFORCE_FILESYSTEM=1 -sASSERTIONS=1 -sERROR_ON_UNDEFINED_SYMBOLS=0 -sEXPORTED_RUNTIME_METHODS=ccallcwrap -sEXPORT_ES6=1 -sMODULARIZE=1 -sEXPORT_NAME='createModule'" cargo build --target=wasm32-unknown-emscripten
	mkdir -p ./out
	#cp ./target/wasm32-unknown-emscripten/debug/deps/interactive.data ./out/interactive.data
	cp ./target/wasm32-unknown-emscripten/debug/interactive.wasm ./out/interactive.wasm
	# cp ./target/wasm32-unknown-emscripten/debug/interactive.wasm.map ./out/interactive.wasm.map
	cp ./target/wasm32-unknown-emscripten/debug/interactive.d ./out/interactive.d
	cp ./target/wasm32-unknown-emscripten/debug/interactive.js ./out/interactive.js
