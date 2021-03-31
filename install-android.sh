#!/usr/bin/env bash

export ANDROID_SDK_ROOT=/usr/lib/android-sdk
mkdir -p $ANDROID_SDK_ROOT

mkdir -p $ANDROID_SDK_ROOT/cmdline-tools/latest

apt-get update && apt-get install -y wget zip android-sdk default-jdk


wget https://dl.google.com/android/repository/commandlinetools-linux-6858069_latest.zip
unzip commandlinetools-linux-6858069_latest.zip
cp -r ./cmdline-tools/*  $ANDROID_SDK_ROOT/cmdline-tools/latest/

cd $ANDROID_SDK_ROOT

export PATH=$PATH:$ANDROID_SDK_ROOT/cmdline-tools/latest/bin:$ANDROID_SDK_ROOT/cmdline-tools/tools/bin

yes | sdkmanager --licenses


mkdir -p $ANDROID_SDK_ROOT/ndk-bundle

wget https://dl.google.com/android/repository/android-ndk-r22b-linux-x86_64.zip
unzip android-ndk-r22b-linux-x86_64.zip

mv android-ndk-r22b/* $ANDROID_SDK_ROOT/ndk-bundle

export NDK_HOME=$ANDROID_SDK_ROOT/ndk-bundle
export PATH=$PATH:$NDK_HOME/build/tools
export PATH=$PATH:$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin

