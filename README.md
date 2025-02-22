# stopssh
An SSH daemon that records all login attempts in detail. Logs to a New Line Delimited JSON file.

```
Usage: stopssh --output <FILE> --listen-addr <ADDR>

Options:
  -o, --output <FILE>
  -l, --listen-addr <ADDR>
  -h, --help                Print help
```

## Logged info
* IP address & port
* SSH client version
* Client's supported algorithms
* SSH public key fingerprint (if any)
* User & password

Eached logged event has a session UUID associated with it. The timestamp is in milliseconds.

## Converting to SQLite
The `json2sql.py` script can convert some JSON logs to an SQLite database:
```
$ python3 json2sql.py ssh_logs.json ssh_logs.db
```
Also check out the `sql_scripts` folder.

## Building
Clone the repository with submodules and build with `cargo`
```
$ git clone --recurse-submodules https://github.com/nagornin/stopssh
$ cd stopssh
$ cargo build --release
```

## Example systemd unit
```ini
[Unit]
Description = "stopssh server"

[Service]
Type=simple
StandardOutput=journal
ExecStart=/home/stopssh/stopssh -l 0.0.0.0:22 -o /home/stopssh/log.json

[Install]
WantedBy=default.target
```

## Example logs
```json
{
  "session_id": "4c857a90-fc4f-45c8-9df0-202a1db38203",
  "time": 1722633399372,
  "event": {
    "type": "tcp_connection",
    "data": {
      "addr": "84.255.37.169:36034"
    }
  }
}
{
  "session_id": "4c857a90-fc4f-45c8-9df0-202a1db38203",
  "time": 1722633399372,
  "event": {
    "type": "version",
    "data": {
      "version": "SSH-2.0-libssh_0.9.6"
    }
  }
}
{
  "session_id":"7ffff3ff-2b61-4f6d-ba5f-9d4d499c3d68",
  "time":1722633313966,
  "event":{
    "type":"password_auth",
    "data":{
      "user":"root",
      "password":"Ae.123456"
    }
  }
}
{
  "session_id": "bc58c1ac-23a5-4e49-80ec-844090765e55",
  "time": 1722642530380,
  "event": {
    "type": "public_key_auth",
    "data": {
      "user": "root",
      "key": "ssh-ed25519 1ntZDbPAjk+6O7u8nEB/XA9y0WNg9VVLbvxiHwQH2Q0"
    }
  }
}
```
