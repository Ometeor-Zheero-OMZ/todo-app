# docker
.PHONY:
build:
	docker-compose up -d --build

.PHONY: build-no-cache
	docker-compose build --no-cache

.PHONY: up
up:
	docker-compose up -d

.PHONY: down
down:
	docker-compose down

.PHONY: stop
stop:
	docker-compose stop

.PHONY: clean
clean:
	docker system prune -a --volumes

# SQLX
.PHONY: create-migrations
create-migrations:
	sqlx migrate add -r init

.PHONY: migrate-up
migrate-up:
	sqlx migrate run --database-url postgres://root:root@localhost:5432/db

.PHONY: migrate-down
migrate-down:
	sqlx migrate revert --database-url postgres://root:root@localhost:5432/db