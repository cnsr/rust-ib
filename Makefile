include .env

run:
	clear
	systemfd --no-pid -s http::8000 -- cargo watch -x run

migrate:
	sqlx migrate run