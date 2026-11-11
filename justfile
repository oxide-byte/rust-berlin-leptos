# Infrastructure services
# -----------------------

infra-up:
	docker compose up -d

infra-down:
	docker compose down

infra-logs:
	docker compose logs -f

infra-restart:
	docker compose restart

# Frontend (client-leptos)
# -------------------------

frontend-build:
	cd client-leptos && trunk build --release

frontend-dev:
	cd client-leptos && trunk serve

frontend-clean:
	rm -rf client-leptos/dist

# Backend (server)
# -----------------

backend-build:
	cd server && cargo build --release

backend-dev:
	cd server && cargo run

backend-clean:
	cd server && cargo clean

# Full build pipeline
# -------------------

build-frontend-to-server: frontend-build
	cp -r client-leptos/dist/* server/static/

build-all: build-frontend-to-server
	cd server && cargo build --release

# Run all infrastructure services (SurrealDB, Keycloak, Mailcatcher)
up:
	docker compose up -d

# Stop all infrastructure services
down:
	docker compose down

# Show available recipes
default:
	@just --list
