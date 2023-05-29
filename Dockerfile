# FROM jkutkut/docker4rust:latest as dev

FROM alpine:3.16 as release

WORKDIR /app
COPY target/release/api_docs_generator /api_docs_generator
ENTRYPOINT ["/api_docs_generator"]