#!/bin/bash -ex

adb root
adb remount

if [ -z "$ANDROID_NDK_HOME" ]; then
    ANDROID_NDK_HOME=$(ls -d /opt/android-ndk-r*/ | tail -1)
    echo "ANDROID_NDK_HOME not set, using ${ANDROID_NDK_HOME}"
fi

ANDROID_NDK_HOME=${ANDROID_NDK_HOME} cargo ndk -t arm64-v8a build --release
adb push ./.target/aarch64-linux-android/release/frontend_hwc3 /vendor/bin/hw/drm_hwcomposer_rs

adb shell stop
adb shell stop vendor.hwcomposer-3
