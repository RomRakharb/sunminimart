FROM registry.fedoraproject.org/fedora

WORKDIR /workdir

RUN dnf upgrade -y && \
    dnf install -y rustup helix nodejs && \
    dnf clean all

RUN rustup-init -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup component add rust-analyzer && \
    rustup target add wasm32-unknown-unknown && \
    cargo install cargo-binstall && \
    cargo binstall -y cargo-leptos leptosfmt

RUN npm install -g vscode-langservers-extracted

EXPOSE 3000
EXPOSE 3001

CMD [ "bash" ]
