#!/usr/bin/env bash
set -x
set -o errexit
brew config;
brew update;
brew install --force curl ninja
pushd $HOME
if [ ! -d "llvm/tools" ]; then rm -rf llvm; git clone -b release_38 --depth=1 https://github.com/llvm-mirror/llvm.git; fi
pushd llvm
pushd tools
if [ ! -d "clang" ]; then git clone -b release_38 --depth=1 https://github.com/llvm-mirror/clang.git; fi
popd
mkdir -p _build
pushd _build
cmake .. -DLIBCLANG_BUILD_STATIC=On -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/clang -DLLVM_TARGETS_TO_BUILD=X86 -GNinja
ninja
sudo mkdir -p /clang
sudo ninja install
sudo cp lib/libclang.a /clang/lib
popd
popd
popd
