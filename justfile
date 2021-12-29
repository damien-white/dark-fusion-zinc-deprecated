set dotenv-load := true

# Helper for running common CI tasks locally
ci:
    cargo fmt --all -- --check && \
    cargo clippy -- --D warnings && \
    cargo test && \
    cargo audit

docker-up:
    docker compose -f "${DOCKER_COMPOSE_FILE}" -p "${DOCKER_PROJECT_NAME}" up -d

docker-down:
    docker-compose -f "${DOCKER_COMPOSE_FILE}" -p "${DOCKER_PROJECT_NAME}" down
