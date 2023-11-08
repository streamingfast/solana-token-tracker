# solana-token-tracker

### Generate protos
```bash
make protogen
```

### Build substreams
```bash
make build
```

### Set up token
Visit https://substreams.streamingfast.io/reference-and-specs/authentication to fetch a token or run below command if you have already followed the instructions from the linked documentation page.
```bash
sftoken
```

### Run substreams
```bash
substreams run substreams.yaml map_holders -e mainnet.sol.streamingfast.io:443 -t +1000
```
