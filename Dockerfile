FROM node:16-bullseye-slim as rust_build

SHELL ["/bin/bash", "-c"]

RUN apt-get update
RUN apt-get install -y \
ca-certificates \
curl \
libcurl3-gnutls \
build-essential

ENV RUSTUP_HOME=/.cargo
ENV CARGO_HOME=/.cargo

# TODO hermetically freeze rust toolchain version
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path
ENV PATH=${CARGO_HOME}/bin:${PATH}

RUN rustup --version
RUN cargo --version
RUN rustc --version

# TODO hermetically freeze wasm-pack version
RUN curl --proto '=https' --tlsv1.2 https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN wasm-pack --version

# TODO heremtically freeze rustwasmc version
# RUN curl https://raw.githubusercontent.com/second-state/rustwasmc/master/installer/init.sh -sSf | sh
# RUN rustwasmc --version




# TODO Split Rust & Node build steps

RUN npm install -g npm@8.10.0
RUN node --version

WORKDIR /myapp

ADD . .
RUN npm ci
RUN npm run build

# TODO build release image

ENTRYPOINT [ "npm" ] 
CMD ["start"]