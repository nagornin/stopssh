import argparse
import sqlite3
import json

parser = argparse.ArgumentParser()

parser.add_argument(metavar="JSON_IN_PATH",
                    help="Path to JSON log file",
                    dest="in_path")

parser.add_argument(
    metavar="SQLITE_OUT_PATH",
    help="Where to save the SQLite database with exported logs",
    dest="out_path")

args = parser.parse_args()
in_file = open(args.in_path)
db = sqlite3.connect(args.out_path)

db.execute("""CREATE TABLE IF NOT EXISTS sessions(
           id TEXT PRIMARY KEY NOT NULL,
           time INTEGER NOT NULL,
           addr TEXT NOT NULL)""")

db.execute("""CREATE TABLE IF NOT EXISTS clients(
           session_id TEXT PRIMARY KEY NOT NULL,
           time INTEGER NOT NULL,
           version TEXT NOT NULL)""")

db.execute("""CREATE TABLE IF NOT EXISTS password_auth(
           session_id TEXT NOT NULL,
           time INTEGER NOT NULL,
           user TEXT NOT NULL,
           password TEXT NOT NULL)""")

for line in in_file:
    log = json.loads(line)
    event = log["event"]
    data = event["data"]

    match event["type"]:
        case "tcp_connection":
            db.execute(
                "INSERT OR IGNORE INTO sessions(id, time, addr) VALUES (?, ?, ?)",
                (log["session_id"], log["time"], data["addr"]))

        case "version":
            db.execute(
                """INSERT OR IGNORE INTO clients(session_id, time, version) VALUES
                       (?, ?, ?)""",
                (log["session_id"], log["time"], data["version"]))

        case "password_auth":
            db.execute(
                """INSERT OR IGNORE INTO password_auth(session_id, time, user,
                       password) VALUES (?, ?, ?, ?)""",
                (log["session_id"], log["time"], data["user"],
                 data["password"]))

db.commit()
