  
FROM ubuntu:20.04

RUN  apt update && apt install -y libx11-dev libvulkan-dev libxcb1-dev xorg-dev 
RUN sudo apt install android-sdk google-android-ndk-installer \
&& export ANDROID_SDK_ROOT="/usr/lib/android-sdk" \
&& export ANDROID_NDK_ROOT="/usr/lib/android-ndk" \
&& export PATH="${PATH}:${ANDROID_SDK_ROOT}/tools/:${ANDROID_SDK_ROOT}/platform-tools/"