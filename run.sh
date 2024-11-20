../fireaelf start substreams-tier1,substreams-tier2 --config-file= \
  --common-live-blocks-addr= --common-first-streamable-block=1 \
  --substreams-state-bundle-size=10 --advertise-chain-name=aelf

substreams run -e localhost:10016 --plaintext \
substreams-aelf-tokens-v0.1.0.spkg \
  map_state_updates -s 58 -t +10