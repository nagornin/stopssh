SELECT version, COUNT() as count FROM (
    SELECT
        version
    FROM sessions
    JOIN clients ON session_id = id
    GROUP BY version, SUBSTR(addr, 1, INSTR(addr, ':') - 1)
) GROUP BY version ORDER BY count DESC