# my_rust_network
Echo server and client for TCP/UDP with Rust.

# How to use server(host)
```bash
# cd socket-programing/
# cargo run tcp server localhost:33333
# cargo run udp server 0.0.0.0:12345
```

# How to use server(client)
```bash
# nc localhost 33333
# cargo run tcp client 127.0.0.1:33333

# nc -u localhost 12345
# cargo run udp client 127.0.0.1:12345
```