SELECT
        DATETIME(MIN(time) / 1000, 'unixepoch', 'localtime') as first_seen,
        DATETIME(MAX(time) / 1000, 'unixepoch', 'localtime') as last_seen,
        SUBSTR(addr, 1, INSTR(addr, ':') - 1) as addr,
        COUNT() as count
    FROM sessions
    GROUP BY SUBSTR(addr, 1, INSTR(addr, ':') - 1)
    ORDER BY count DESC