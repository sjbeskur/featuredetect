FROM ubuntu:latest

RUN apt update -y
RUN apt install git -y
RUN apt install build-essential cmake curl -y
RUN apt install g++-aarch64-linux-gnu clang libclang-dev -y

RUN git clone https://github.com/opencv/opencv.git --depth 1
WORKDIR /opencv 


RUN cmake -B build -S. -DCMAKE_BUILD_TYPE=RELEASE -DCMAKE_INSTALL_PREFIX=/usr/local \
    -DOPENCV_GENERATE_PKGCONFIG=ON  \
    -DWITH_QT=OFF \
    -DBUILD_TESTS=OFF \
    -DBUILD_PERF_TESTS=OFF \
    -DBUILD_EXAMPLES=OFF \
    -DCMAKE_TOOLCHAIN_FILE=../platforms/linux/aarch64-gnu.toolchain.cmake .. \
    && cd build && make -j16 && make install

WORKDIR /opencv/build

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="${PATH}:/root/.cargo/bin"
WORKDIR /build
RUN ~/.cargo/bin/rustup target add aarch64-unknown-linux-gnu 

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=/usr/bin/aarch64-linux-gnu-gcc
#RUN cargo build --target aarch64-unknown-linux-gnu
RUN echo $PATH
ENTRYPOINT [ "cargo", "build" ]
CMD ["--target", "aarch64-unknown-linux-gnu"]

# docker run -v $(pwd):/build builder
# docker run -u $(id -u ${USER}):$(id -g ${USER}) -v $(pwd):/build builder
