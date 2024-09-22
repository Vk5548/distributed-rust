# the official Rust image
FROM rust:latest

# Setting the working directory
WORKDIR /app

# Copy the source code into the container
COPY . .

# Building the project
RUN cargo build --release

#Expose the neccessary port
EXPOSE 8080

# Start the server
CMD ["./target/release/simple_distribution_system"]