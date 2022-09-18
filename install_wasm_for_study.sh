#!/usr/bin/env bash
# ---
# Project : Auto Env Install to WebAssembly Studies
# Version : 0.0.1
# Date    : 2022-09-18
# Author  : Talles H. < koureimakoto+autowasmenv@gmail.com >
# License : GNU/ GPL v3.0
# ---
# Use: You can use freely to study WebAssembly 


INTERN_SHELL_TYPE=''


echo "Init Auto Env to study WebAssembly"

if   [[ $SHELL == '/usr/bin/zsh'  ]]; then
    INTERN_SHELL_TYPE="$HOME/.zshrc"
elif [[ $SHELL == '/usr/bin/bash' ]]; then
    INTERN_SHELL_TYPE="$HOME/.bashrc"
else
    INTERN_SHELL_TYPE='undf'
fi

echo 'Installing EMscriptem'

# Get the emsdk repo
git clone https://github.com/emscripten-core/emsdk.git

# Enter that directory
cd emsdk

# Download and install the latest SDK tools.
./emsdk install latest

# Make the "latest" SDK "active" for the current user. (writes .emscripten file)
./emsdk activate latest

# Activate PATH and other environment variables in the current terminal
source ./emsdk_env.sh

if [[ $INTERN_SHELL_TYPE != 'undf' ]]; then
    printf "source $PWD/emsdk_env.sh\n" >> "$HOME/.$INTERN_SHELL_TYPE"
fi

# IF EMSDK NOT SUPRESS THE VERBOSE 
# SET in  ~/emsdk/emsdk.py
# line: QUIET = int(...) TO QUIET 1 or def info(msg) if [remove 'not'] QUIET

echo "Installing WAT COMPILER"

echo " >> Downloading from repository"
git clone --recursive https://github.com/WebAssembly/wabt

echo " >> Updating submodules"
cd wabt
git submodule update --init

echo " >> Building::cmake"
mkdir build
cd build
cmake ..
cmake --build .


LINEBREAK="\n"
echo " Installed WAT COMPILER"

if [[ INTERN_SHELL_TYPE != 'undf' ]]; then
    echo   " >> Creating Alias "
    printf "# -- WebAssembly Text Alias --  "             $LINEBREAK >> $INTERN_SHELL_TYPE
    printf "alias -g wat2wasm=\"$PWD/\wat2wasm\""         $LINEBREAK >> $INTERN_SHELL_TYPE 
    printf "alias -g wasm-objdump=\"$PWD/\wasm-objdump\"" $LINEBREAK >> $INTERN_SHELL_TYPE
    printf "alias -g wasm2wat=\"$PWD/\wasm2wat\""         $LINEBREAK >> $INTERN_SHELL_TYPE
else
    echo " >> Undefined Shell "
    echo " >> You need to created manually the aliases"
    echo " Add the following alias in your shell shettings " $LINEBREAK \
         "alias -g wat2wasm=\"$PWD/\wat2wasm\""              $LINEBREAK \
         "alias -g wasm-objdump=\"$PWD/\wasm-objdump\""      $LINEBREAK \
         "alias -g wasm2wat=\"$PWD/\wasm2wat\""              $LINEBREAK 