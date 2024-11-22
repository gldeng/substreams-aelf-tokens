substreams run -e localhost:10016 --plaintext \
substreams-aelf-tokens-v0.1.0.spkg \
  events_of_transfer -s 139 -t +1

substreams run -e localhost:10016 --plaintext \
substreams-aelf-tokens-v0.1.0.spkg \
  state_updates_of_balance -s 139 -t +1

substreams run -e localhost:10016 --plaintext \
  substreams-aelf-tokens-v0.1.0.spkg \
    all_transfers -s 139 -t +1

substreams run -e localhost:10016 --plaintext \
substreams-aelf-tokens-v0.1.0.spkg \
  all_balance_updates -s 139 -t +1

