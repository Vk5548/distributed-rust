# the official Rust image
FROM rust:latest

# Setting the working directory
WORKDIR /app

# Copy the source code into the container
COPY . .

# Building the project ; will build teh srever binary : main.rs
RUN cargo build --release

#Expose the neccessary port for the servers (which is different for each container)
# EXPOSE 8081 8082 8083

#Expose the neccessary port for the servers , when communication is inter-container (which is different for each container)
EXPOSE 8080

# Start the server ; suitable for production environment
# CMD ["./target/release/simple_distribution_system"] 
# This abovecommand directly executes the compiled binary (the executable file) located at ./target/release/simple_distribution_system.

# Run the servers (this runs the server logic)
CMD ["cargo", "run", "--release"]