SELECT
        DATETIME(
            password_auth.time / 1000.0, 'unixepoch', 'subsec', 'localtime'
        ) as time,
        SUBSTR(addr, 1, INSTR(addr, ':') - 1) as addr,
        user,
        password,
        version
    FROM sessions
    JOIN password_auth ON password_auth.session_id = id
    JOIN clients ON clients.session_id = id
    ORDER BY password_auth.time ASC
