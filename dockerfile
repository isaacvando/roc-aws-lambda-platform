FROM ubuntu:latest

# Update the package lists
RUN apt-get update
RUN apt-get -y install curl
# RUN apt-get install -y python3-pip
# RUN pip

# Install rustup and cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
# RUN cargo binstall cargo-lambda

# RUN curl -LsSf https://astral.sh/uv/install.sh | sh
# RUN /bin/bash -c "source ~/.bashrc"
# RUN uv pip install cargo-lambda

RUN curl -OL https://github.com/roc-lang/roc/releases/download/nightly/roc_nightly-linux_x86_64-latest.tar.gz && tar -xf roc_nightly-linux_x86_64-latest.tar.gz
RUN echo "hi"
RUN mv $(ls | grep "^roc_nightly-") /root/bin/sh
# ENV PATH="/root/src:${PATH}"
RUN roc --version

RUN cargo --help
COPY build.sh /build.sh
ENTRYPOINT ["/build.sh"]