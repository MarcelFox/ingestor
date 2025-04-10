watch:
	cargo lambda watch

hello:
	cargo lambda invoke ingestor --data-ascii '{"firstName": "John"}'

event:
	cargo lambda invoke ingestor --data-file ./event-data.json
