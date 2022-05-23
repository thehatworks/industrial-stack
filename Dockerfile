FROM node:16-bullseye-slim as build

ARG NODE_ENVIRONMENT=production

# Rust from scratch

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

# Verify Rust toolchain configured properly
RUN rustup --version
# rustup 1.24.3 (ce5817a94 2021-05-31)
RUN cargo --version
# cargo 1.60.0 (d1fd9fe 2022-03-01)
RUN rustc --version
# rustc 1.60.0 (7737e0b5c 2022-04-04)

RUN rustup target add wasm32-unknown-unknown

# TODO hermetically freeze wasm-pack version
RUN curl --proto '=https' --tlsv1.2 https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN wasm-pack --version

RUN npm install -g npm@8.10.0
RUN npm --version
# check 8.10.0

WORKDIR /myapp
ADD . .

# We want dev dependencies here, so keep NODE_ENV unset until after npm ci
RUN npm ci
ENV NODE_ENV=${NODE_ENVIRONMENT}
RUN npm run build

FROM denoland/deno:1.21.3 as deploy

USER deno

RUN deno install --allow-read --allow-write --allow-env --allow-net --allow-run --no-check -r -f https://deno.land/x/deploy/deployctl.ts

WORKDIR /myapp

COPY --from=build /myapp/build build
COPY --from=build /myapp/public public

RUN deployctl deploy --prod --include=build,public,rustlib --project=industrial-stack ./build/index.js

# FROM denoland/deno:1.21.3 as run

# EXPOSE 8000

# # Best security practices
# USER deno

# ENV NODE_ENV=${NODE_ENVIRONMENT}

# WORKDIR /myapp

# COPY --from=build /myapp/build build
# COPY --from=build /myapp/public public

# RUN deno --version
# # deno 1.21.3 (release, x86_64-unknown-linux-gnu)
# # v8 10.0.139.17
# # typescript 4.6.2

# ENTRYPOINT [ "deno" ]
# CMD ["run", "--unstable", "--allow-net", "--allow-read", "--allow-env", "./build/index.js"]