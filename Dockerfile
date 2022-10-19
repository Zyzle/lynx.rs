FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/lynx /usr/local/bin/lynx

RUN apt-get update \
	&& apt-get install -y ca-certificates tzdata \
	&& rm -rf /var/lib/apt/lists/*
	
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["lynx"]