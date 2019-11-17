# cc_tuts
Hello holo :D

## compile
```bash
hc package
```

## run server
```bash
hc run -i http
```

## run tests
```bash
hc package # compile first
hc test # run test

# "MANUAL" WAY OF TESTING
# run server in http mode (tests make http requests)
hc run -i http
# test using npm (navigate to test folder first!)
npm start
```

## test
```bash
curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "hello", "function": "hello_holo", "args": {"name": "Insert Your Name"} }}' http://127.0.0.1:8888 | jq
curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "hello", "function": "generate_rand", "args": {} }}' http://127.0.0.1:8888 | jq
```
