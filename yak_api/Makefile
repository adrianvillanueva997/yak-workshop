infra-up:
	docker-compose -f infra/docker-compose.yml up -d
infra-down:
	docker-compose -f infra/docker-compose.yml down
hadolint:
	hadolint --config hadolint.yml Dockerfile
dev:
	export DATABASE_URL=postgres://postgres:postgres@localhost:5432/yak_api
	export
	cargo run