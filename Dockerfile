# Use the minimal image
FROM rust:1.48 AS build

# Where we will build the program
WORKDIR /src/helloworld

# Install Mysql Client
RUN apt-get update && \
    apt-get install -y default-libmysqlclient-dev libssl-dev

# Install Cargo-make
RUN cargo install cargo-make

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Copy source code into the container
COPY . .

# Build the program in release mode
RUN cargo make

# Create the runtime image
FROM ubuntu:18.04

# Install Mysql Client
RUN apt-get update && \
    apt-get install -y libmariadb-dev

# Copy the compiled service binary
COPY --from=build /src/helloworld/target/debug/backend /usr/local/bin/backend
COPY --from=build /src/helloworld/frontend/static /usr/local/bin/static
COPY --from=build /src/helloworld/frontend/index.html /usr/local/bin/static/index.html

WORKDIR /usr/local/bin

# Start the helloworld service on container boot
CMD ["backend"]
