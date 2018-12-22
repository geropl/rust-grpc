FROM gitpod/workspace-full:latest

USER root
RUN mkdir -p proto-download && \
    cd proto-download && \
    wget https://github.com/protocolbuffers/protobuf/releases/download/v3.6.1/protoc-3.6.1-linux-x86_64.zip -O protoc.zip && \
    unzip protoc.zip && \
    mv bin/protoc /usr/bin/protoc && \
    mv include/* /usr/local/include/ && \
    cd .. && \
    rm -Rf proto-download;
