../fireaelf start substreams-tier1,substreams-tier2 --config-file= \
  --common-live-blocks-addr= --common-first-streamable-block=1 \
  --substreams-state-bundle-size=10 --advertise-chain-name=aelf

substreams run -e localhost:10016 --plaintext \
substreams-aelf-tokens-v0.1.0.spkg \
  filtered_transactions -s 139 -t +1

substreams run -e localhost:10016 --plaintext \
substreams-aelf-tokens-v0.1.0.spkg \
  all_events -s 139 -t +1

substreams run -e localhost:10016 --plaintext \
substreams-aelf-tokens-v0.1.0.spkg \
  filtered_events -s 139 -t +1

substreams run -e localhost:10016 --plaintext \
  substreams-aelf-tokens-v0.1.0.spkg \
    all_transfers -s 139 -t +1

substreams run -e localhost:10016 --plaintext \
substreams-aelf-tokens-v0.1.0.spkg \
  all_balance_changes -s 139 -t +1

substreams run -e localhost:10016 --plaintext \
substreams-aelf-tokens-v0.1.0.spkg \
  filtered_state_updates -s 139 -t +1