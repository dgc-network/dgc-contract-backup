# Copyright (c) The dgc.network
# SPDX-License-Identifier: Apache-2.0

FROM ubuntu:bionic

RUN apt-get update \
 && apt-get install gnupg -y

RUN echo "deb [arch=amd64] http://repo.sawtooth.me/ubuntu/ci bionic universe" >> /etc/apt/sources.list \
 && apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys 8AA7AF1F1091A5FD \
 && apt-get update \
 && apt-get install -y -q \
    build-essential \
    git \
    latexmk \
    libffi-dev \
    libssl-dev \
    libzmq3-dev \
    python3-pip

RUN apt-get update && apt-get install -y -q --no-install-recommends \
    curl \
 && curl -s -S -o /tmp/setup-node.sh https://deb.nodesource.com/setup_6.x

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y -q \
    dvipng \
    make \
    pkg-config \
    sudo \
    texlive-fonts-recommended \
    texlive-latex-base \
    texlive-latex-extra \
    texlive-latex-recommended \
    zip \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && pip3 install \
    sphinx \
    sphinxcontrib-httpdomain \
    sphinxcontrib-openapi \
    sphinx_rtd_theme

RUN curl -OLsS https://github.com/google/protobuf/releases/download/v3.5.1/protoc-3.5.1-linux-x86_64.zip \
 && unzip protoc-3.5.1-linux-x86_64.zip -d protoc3 \
 && rm protoc-3.5.1-linux-x86_64.zip

RUN curl https://sh.rustup.rs -sSf > /usr/bin/rustup-init \
 && chmod +x /usr/bin/rustup-init \
 && rustup-init -y

ENV PATH=$PATH:/protoc3/bin:/root/.cargo/bin:/project/docs/target/debug \
    CARGO_INCREMENTAL=0

WORKDIR /project/docs
