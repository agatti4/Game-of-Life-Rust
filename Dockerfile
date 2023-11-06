# Use a Rust base image
FROM rust:latest

# Set the working directory
WORKDIR /app

# Set the TERM environment variable
ENV TERM xterm

# Copy your Rust source code into the container
COPY . /app

# Build your Rust program
RUN cargo build --release

# Define the entry point for your program
ENTRYPOINT ["target/release/gol"]

# Set default command line arguments
CMD ["gliderGun.txt", "wrap", "show", "fast"]
