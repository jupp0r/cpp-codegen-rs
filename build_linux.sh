#!/usr/bin/env bash
set -x
apt-get update --yes
apt-get install cmake git python g++ libc++-dev ninja-build zlib1g-dev libncursesw5-dev libffi-dev software-properties-common python-software-properties curl --yes
apt-add-repository "deb http://apt.llvm.org/trusty/ llvm-toolchain-trusty-3.8 main" --yes;
curl http://apt.llvm.org/llvm-snapshot.gpg.key | sudo apt-key add -
sudo apt-get update --yes
sudo apt-get install llvm-3.8-dev --yes
pushd /tmp
git clone -b release_38 --depth=1 https://github.com/llvm-mirror/llvm.git
pushd llvm
pushd tools
git clone -b release_38 --depth=1 https://github.com/llvm-mirror/clang.git
popd
mkdir -p _build
pushd _build
cmake .. -DLIBCLANG_BUILD_STATIC=On -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/clang -DLLVM_TARGETS_TO_BUILD=X86 -GNinja
ninja
ninja install
cp tools/clang/lib/libclang.a /clang/lib
popd
popd
popd
