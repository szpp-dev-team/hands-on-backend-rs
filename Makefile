.PHONY: rund
rund: 
	docker compose up -d --build

.PHONY: db/init
db/init:
	diesel database setup \
		--database-url postgres://szpp:szpp3776@localhost:5432/szpp-mini-judge
	diesel migration run \
		--database-url postgres://szpp:szpp3776@localhost:5432/szpp-mini-judge

.PHONY: db/migrate
db/migrate:
	diesel migration revert --all \
		--database-url postgres://szpp:szpp3776@localhost:5432/szpp-mini-judge
	diesel migration run \
		--database-url postgres://szpp:szpp3776@localhost:5432/szpp-mini-judge
	diesel database setup \
		--database-url postgres://szpp:szpp3776@localhost:5432/szpp-mini-judge

.PHONY: db/seed
db/seed: db/migrate
	docker cp ./test/db szpp-mini-judge-db:/tmp/testdata
	docker exec -e PGPASSWORD=szpp3776 szpp-mini-judge-db psql -d szpp-mini-judge -U szpp \
		-c "COPY users FROM '/tmp/testdata/users.csv' csv header encoding 'UTF8'" \
		-c "COPY contests FROM '/tmp/testdata/contests.csv' csv header encoding 'UTF8'" \
		-c "COPY tasks FROM '/tmp/testdata/tasks.csv' csv header encoding 'UTF8'"
