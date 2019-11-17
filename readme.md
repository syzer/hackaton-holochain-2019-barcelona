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

## basic tests (CURL)
```bash
# START APP
hc run -i http -p 8888

# different http requests
curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "hello", "function": "hello_holo", "args": {"name": "Insert Your Name"} }}' http://127.0.0.1:8888 | jq
curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "hello", "function": "generate_rand", "args": {} }}' http://127.0.0.1:8888 | jq
curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "hello", "function": "create_message", "args": {"message": { "content": "hello world!! first message"} } }}' http://127.0.0.1:8888 | jq
curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "hello", "function": "retrieve_message", "args": {"address": "QmZNDbibhFVMnPDFomEfbsX1PShYjR6aFRoDs4ikxThiNn" } }}' http://127.0.0.1:8888 | jq
```

### advanced tests (2 agents)
```bash
# start agent 1 (3401)
# hc run -i http -p 3401 # NOT WORKING
holochain -c conductor-config-bob.toml
# start agent 2 (3402)
# hc run -i http -p 3402 # NOT WORKING
holochain -c conductor-config-alice.toml

# TESTS-------------------------------------------------------------------------
# put message in DHT from agent 1 (3401)
curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "hello", "function": "create_message", "args": {"message": { "content": "hello from port 3401"} } }}' http://127.0.0.1:3401 | jq
# => Qmanzk2aQs73tsyoioidFB9Yww8PwbFYKF8bXJyrbD3x2b
# retrieve message from agent1 (3401)
curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "hello", "function": "retrieve_message", "args": {"address": "Qmanzk2aQs73tsyoioidFB9Yww8PwbFYKF8bXJyrbD3x2b" } }}' http://127.0.0.1:3401 | jq
# retrieve message from agent2 (3402)
curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "hello", "function": "retrieve_message", "args": {"address": "Qmanzk2aQs73tsyoioidFB9Yww8PwbFYKF8bXJyrbD3x2b" } }}' http://127.0.0.1:3402 | jq
```

