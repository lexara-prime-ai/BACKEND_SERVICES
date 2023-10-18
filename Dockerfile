# Specify build image
FROM rust:latest

# Specify working directory
WORKDIR /app/

# Copy Source files over to new directory
COPY . .

# Specify commands to run
# Specify the default toolchain
RUN rustup default 

# Install 'diesel-cli' in the running instance
RUN cargo install diesel-cli --no-default-features --features postgres

# Install additional tooling
RUN cargo install cargo-watch

# Expose the specified port
EXPOSE 8000 # rocket's default port

# Specify commands to run after initial setup
CMD ["cargo", "watch", "--why", "--", "echo"]