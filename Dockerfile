# Build step
FROM rust:1.43 as builder
WORKDIR /usr/src/ablog-api-local
COPY . .
RUN cargo install --path .

# Serving container
FROM debian:buster-slim
WORKDIR /usr/src/ablog-api-local
RUN apt-get update && apt-get install -y wget texlive-latex-base texlive-fonts-recommended texlive-fonts-extra texlive-latex-extra
RUN wget https://github.com/jgm/pandoc/releases/download/2.9.2.1/pandoc-2.9.2.1-1-amd64.deb && \
  dpkg -i pandoc-2.9.2.1-1-amd64.deb;

COPY . .
COPY --from=builder /usr/local/cargo/bin/ablog-api-local /usr/local/bin/ablog-api-local
CMD ["ablog-api-local"]