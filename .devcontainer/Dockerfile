ARG RUSTVERSION=latest
FROM rust:$RUSTVERSION

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Create the user
RUN groupadd --gid $USER_GID $USERNAME 
run useradd --uid $USER_UID --gid $USER_GID -m $USERNAME 

USER $USERNAME

RUN rustup $RUSTVERSION default \ 
    && rustup component add clippy rustfmt

ENV CARGO_HOME=/home/$USERNAME/.cargo
ENV SHELL=/bin/bash

RUN cargo install cargo-cache cargo-criterion \
    && cargo cache -a