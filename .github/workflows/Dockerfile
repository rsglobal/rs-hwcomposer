FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

# Taking into account layer structure, everything should be done within one layer.
RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y clang-15 clang-tidy-15 git clang llvm make wget sudo rsync lld curl unzip

ENV USER_HOME /github/home

RUN mkdir -pv ${USER_HOME}

# Create project path
RUN mkdir -pv /github/workspace
WORKDIR /github/workspace

RUN git config --global user.name "FIRST_NAME LAST_NAME" && git config --global user.email "MY_NAME@example.com"

# Run CI:
CMD cd /github/workspace && \
    curl https://sh.rustup.rs -sSf | bash -s -- -y && \
    . ${USER_HOME}/.cargo/env && \
    rustup default nightly && rustup target add aarch64-linux-android && \
    git config --global --add safe.directory /github/workspace && \
    git config --global -l && \
    cargo install cargo-ndk && \
    wget https://dl.google.com/android/repository/android-ndk-r26d-linux.zip -O /tmp/ndk.zip && \
    mkdir -p /opt/ndk && \
    unzip /tmp/ndk.zip -d /opt/ && \
    export RUSTFLAGS="-D warnings" && cargo check && cargo test && cargo fmt --check && cargo clippy && \
    ANDROID_NDK_HOME=/opt/android-ndk-r26d/ cargo ndk -t arm64-v8a build --release && \
    find && \
    echo "\n\e[32m --- SUCCESS ---\n"
