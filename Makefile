build:
	docker-compose build

run:
	docker-compose up

down:
	docker-compose down

shell:
	docker-compose run rust-rsa-replacer /bin/sh

access:
	docker-compose exec -it rust-rsa-replacer /bin/sh

add:
	docker-compose run rust-rsa-replacer cargo add ${package}
