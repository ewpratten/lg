FROM debian:latest
ENV ROCKET_ADDRESS 0.0.0.0
ENV ROCKET_PORT 80

# Install needed networking tools
RUN apt-get update -y
RUN apt-get install -y iputils-ping
RUN apt-get install -y net-tools
RUN apt-get install -y traceroute

# Copy in the application binary
COPY ./target/release/lookingglass /lookingglass

# Run the application
ENTRYPOINT [ "/lookingglass", "/config/local.json", "/config/global.json" ]