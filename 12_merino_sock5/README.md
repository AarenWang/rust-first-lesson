```shell
export RUST_LOG=trace
cargo run --package merino --bin merino -- --no-auth

# 用curl请求
curl -v --proxy socks5://127.0.0.1:1090 https://www.baidu.com
```


一次代理请求 输出日志
```
 2022-01-26T11:48:32.310 TRACE mio::poll > registering event source with poller: token=Token(16777218), interests=READABLE | WRITABLE
 2022-01-26T11:48:32.310 DEBUG merino    > New connection
 2022-01-26T11:48:32.310 TRACE merino    > Version: 5 Auth nmethods: 2
 2022-01-26T11:48:32.310 DEBUG merino    > Authenticating
 2022-01-26T11:48:32.310 TRACE merino    > methods: [0]
 2022-01-26T11:48:32.310 DEBUG merino    > Sending NOAUTH packet
 2022-01-26T11:48:32.311 DEBUG merino    > NOAUTH sent
 2022-01-26T11:48:32.311 DEBUG merino    > Starting to relay data
 2022-01-26T11:48:32.311 TRACE merino    > Server waiting for connect
 2022-01-26T11:48:32.314 TRACE merino    > Server received [5, 1, 0, 1]
 2022-01-26T11:48:32.314 TRACE merino    > Getting Addr
 2022-01-26T11:48:32.314 INFO  merino    > New Request: Command: Connect Addr: 112.80.248.76, Port: 443
 2022-01-26T11:48:32.314 DEBUG merino    > Handling CONNECT Command
 2022-01-26T11:48:32.314 TRACE merino    > Connecting to: [112.80.248.76:443]
 2022-01-26T11:48:32.314 TRACE mio::poll > registering event source with poller: token=Token(16777219), interests=READABLE | WRITABLE
 2022-01-26T11:48:32.340 TRACE merino    > Connected!
 2022-01-26T11:48:32.340 TRACE merino    > copy bidirectional
 2022-01-26T11:48:32.425 TRACE merino    > already closed
 2022-01-26T11:48:32.425 TRACE mio::poll > deregistering event source from poller
 2022-01-26T11:48:32.425 TRACE mio::poll > deregistering event source from poller
```