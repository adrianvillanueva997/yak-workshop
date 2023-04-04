# These environment variables are just for local development
export POSTGRES_URL=postgres://postgres:postgres@192.168.1.31:5432/yak_api
export DATABASE_URL=postgres://postgres:postgres@192.168.1.31:5432/yak_api
export REDIS_URL=redis://192.168.1.31:6379

infra-up:
	podman-compose -f infra/docker-compose.yml up -d
infra-down:
	podman-compose -f infra/docker-compose.yml down
hadolint:
	hadolint --config .hadolint.yml Dockerfile
dev:
	cargo run
migrate:
	sqlx database drop && sqlx database create && sqlx migrate run
