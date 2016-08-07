#!/usr/bin/env bash
set -x
set -o errexit
sudo apt-get update --yes
sudo apt-get install cmake git python g++ libc++-dev ninja-build zlib1g-dev libncursesw5-dev libffi-dev software-properties-common python-software-properties curl --yes
sudo apt-add-repository "deb http://apt.llvm.org/trusty/ llvm-toolchain-trusty-3.8 main" --yes;
curl http://apt.llvm.org/llvm-snapshot.gpg.key | sudo apt-key add -
sudo apt-get update --yes
sudo apt-get install llvm-3.8-dev --yes
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
