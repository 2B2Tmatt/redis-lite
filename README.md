# Redis Lite

A lightweight Redis-like key-value store in Rust. Supports basic operations like `SET`, `GET`, `SETEX`, `EXPIRE`, and `EXISTS`. Built for learning. 

## Implementations

* `SET key value` – Set a key to a value.
* `GET key` – Retrieve the value of a key.
* `SETEX key seconds value` – Set a key with an expiration time.
* `EXPIRE key seconds` – Set an expiration time on an existing key.
* `EXISTS key` – Check if a key exists.
* Custom TCP port support via command-line arguments.

## Usage

1. Clone the repository:

```bash
git clone https://github.com/2B2Tmatt/redis-lite.git
cd redis-lite
```

2. Run the server:

```bash
cargo run [PORT]
```

* Replace `[PORT]` with the desired port number (default: `6379`).

Example:

```bash
cargo run 6380
```

3. Connect and test (example with `nc`):

```bash
nc 127.0.0.1 6380
SET mykey hello
GET mykey
EXISTS mykey
SETEX tempkey 10 world
EXPIRE mykey 30
```
Example Usage: 
<img width="1919" height="1073" alt="image" src="https://github.com/user-attachments/assets/ec136330-1228-4316-93c2-ac809047589e" />


## Architecture

* Rust standard library based TCP server.
* Handles concurrent connections safely and stores data in an `Arc<Mutex<HashMap<String, Data>>>`.
* Minimal and fast.
