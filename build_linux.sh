#!/usr/bin/env bash
set -x
set -o errexit
sudo apt-get update --yes
sudo apt-get install git python2.7 libpython2.7-dev g++ libc++-dev ninja-build zlib1g-dev libncursesw5-dev libffi-dev software-properties-common python-software-properties curl --yes
sudo apt-get install -y build-essential
wget https://cmake.org/files/v3.7/cmake-3.7.2.tar.gz
tar xf cmake-3.7.2.tar.gz
cd cmake-3.7.2
./configure
make -j `nproc`
sudo make install
pushd $HOME
if [ ! -d "llvm/tools" ]; then rm -rf llvm; git clone -b release_39 --depth=1 https://github.com/llvm-mirror/llvm.git; fi
pushd llvm
pushd tools
if [ ! -d "clang" ]; then git clone -b release_39 --depth=1 https://github.com/llvm-mirror/clang.git; fi
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
