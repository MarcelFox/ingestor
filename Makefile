watch:
	cargo lambda watch

hello:
	cargo lambda invoke ingestor --data-ascii '{"firstName": "John"}'

event:
	cargo lambda invoke ingestor --data-file ./event-data.json

build:
	cargo lambda build --target x86_64-unknown-linux-musl --release --output-format zip;
	unzip -o target/lambda/ingestor/bootstrap.zip -d .;

run\:watch:
	cargo lambda watch;

run\:docker:
	$(MAKE) build;
	docker compose up --build;
