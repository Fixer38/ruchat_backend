FROM rust:1.49

# Equivalent to 'cd app'
# Dir will be created if not created
WORKDIR app

# Copy all files from working environment to the Docker image
COPY . .

# Forces sqlx to use the offline feature
# Now relies on 'sqlx-data.json' to migrate
ENV SQLX_OFFLINE true

# Build the Rust binary
# Release profile used to make it faster
RUN cargo build --release

# When 'docker run' is executed, launch the binary located in the release folder
ENTRYPOINT ["./target/release/ruchat_backend"]