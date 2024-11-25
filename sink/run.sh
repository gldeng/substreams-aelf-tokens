
substreams-sink-sql setup "psql://dev-node:insecure-change-me-in-prod@127.0.0.1:5432/substreams_example?sslmode=disable" \
./sink/substreams.dev.yaml

substreams-sink-sql run "psql://dev-node:insecure-change-me-in-prod@127.0.0.1:5432/substreams_example?sslmode=disable" \
--endpoint localhost:10016 \
--plaintext \
--flush-interval 1 \
./sink/substreams.dev.yaml