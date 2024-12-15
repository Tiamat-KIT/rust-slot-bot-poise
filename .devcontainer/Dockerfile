FROM rust:1.83.0-bullseye

ARG USER_NAME=utakata
ARG USER_ID=1000
ARG GROUP_ID=1000

RUN apt update -y \
    && apt upgrade -y \
    && apt install git curl zip unzip -y \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -g ${GROUP_ID} ${USER_NAME} && \
    useradd -m -u ${USER_ID} -g ${USER_NAME} ${USER_NAME}   

# デフォルトシェルをbashに設定
RUN chsh -s /bin/bash ${USER_NAME}

# Set working directory
# WORKDIR /workspace
# Bunのインストール (ユーザー専用)
# USER ${USER_NAME}
# RUN curl -fsSL https://bun.sh/install | bash
# ENV PATH="/home/${USER_NAME}/.bun/bin:${PATH}"
# RUN echo 'export PATH="/home/${USER_NAME}/.bun/bin:$PATH"' >> /home/${USER_NAME}/.bashrc

# RUN curl https://get.volta.sh | bash \
#     && export VOLTA_HOME="$HOME/.volta" \
#     && export PATH="$VOLTA_HOME/bin:$PATH" \
#     && volta install node 

USER root
RUN chown -R ${USER_NAME}:${USER_NAME} /usr/local/cargo \
    && chmod -R g+rw /usr/local/cargo \
    && chmod -R g+rw /usr/local/lib/

RUN rustup target add wasm32-unknown-unknown \
    && cargo install cargo-shuttle
# 最後にユーザーを切り替え
USER ${USER_NAME}
# Set working directory
WORKDIR /workspace
RUN chown -R ${USER_NAME}:${USER_NAME} /workspace