.PHONY: dev-frontend dev-server entities prod

prod:
	@cd frontend && trunk build --release
	@cargo build --release

dev-frontend:
	@cd frontend && trunk serve --proxy-backend=http://[::1]:8081/api/

dev-server:
	@PORT=8081 cargo watch -- cargo run --bin server

entities:
	@sea-orm-cli generate entity -u sqlite://reader.db --with-serde both -o entity/src/ -l

# cargo install cross
cross:
	@cross build --release --target=arm-unknown-linux-gnueabihf --bin server
