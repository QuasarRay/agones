SHELL := /usr/bin/env bash

.PHONY: generate test validate up down logs clean joern-shell status

generate:
	./generate.sh

test:
	docker compose run --rm --entrypoint python3 converter -m unittest -v test_graphml_to_grafana.py

validate: test
	docker compose config -q

up:
	@test -f out/grafana/manifest.json || (echo "Missing out/grafana/manifest.json; run 'make generate' first." >&2; exit 1)
	docker compose up -d cpg-data grafana

down:
	docker compose down

logs:
	docker compose logs -f cpg-data grafana

status:
	docker compose ps

joern-shell:
	@mkdir -p out
	docker compose run --rm --entrypoint /bin/bash joern

clean:
	rm -rf out
