build:
	docker-compose build

run:
	docker-compose up

down:
	docker-compose down

shell:
	docker-compose run rust-template /bin/sh

access:
	docker-compose exec -it rust-template /bin/sh

add:
	docker-compose run rust-img-cropper cargo add ${package}
