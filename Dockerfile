FROM rustlang/rust:nightly

RUN apt-get -y update && \
    apt-get -y install make gcc curl gnupg1 gnupg2 gosu sudo && \
    echo "deb http://apt.llvm.org/xenial/ llvm-toolchain-xenial-6.0 main" >> /etc/apt/sources.list && \
    echo "deb-src http://apt.llvm.org/xenial/ llvm-toolchain-xenial-6.0 main" >> /etc/apt/sources.list && \
    curl  https://apt.llvm.org/llvm-snapshot.gpg.key| apt-key add - && \
    apt-get -y install clang-6.0 lldb-6.0 lld-6.0 qemu-system-i386

RUN cargo install cargo-xbuild
RUN rustup component add rust-src

COPY entrypoint.sh /usr/local/bin/

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
CMD ["/bin/bash"]
