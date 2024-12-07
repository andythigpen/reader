.PHONY: dev-frontend dev-server entities prod prod-frontend prod-server cross arm

arm: prod-frontend cross

prod: prod-frontend prod-server

prod-frontend:
	@cd frontend && trunk build --release

prod-server:
	@cargo build --release --bin server

dev-frontend:
	@cd frontend && trunk serve --proxy-backend=http://[::1]:8081/api/

dev-server:
	@PORT=8081 cargo watch -- cargo run --bin server

entities:
	@sea-orm-cli generate entity -u sqlite://data/reader.db --with-serde both -o server/entity/src/ -l

# cargo install cross
cross:
	@cross build --release --target=arm-unknown-linux-gnueabihf --bin server
