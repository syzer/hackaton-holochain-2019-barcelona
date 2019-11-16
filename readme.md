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

## test
```bash
curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "hello", "function": "hello_holo", "args": {} }}' http://127.0.0.1:8888
```
