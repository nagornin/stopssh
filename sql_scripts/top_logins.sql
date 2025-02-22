SELECT user, password, COUNT() as count FROM (
    SELECT user, password
    FROM sessions
    JOIN password_auth ON session_id = id
    GROUP BY user, password, SUBSTR(addr, 1, INSTR(addr, ':') - 1)
) GROUP BY user, password ORDER BY count DESC