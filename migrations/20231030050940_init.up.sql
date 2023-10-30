-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


CREATE TABLE
    IF NOT EXISTS jury (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        part_no VARCHAR(255) NOT NULL UNIQUE,
        pool_no TEXT NOT NULL,
        review_type TEXT NOT NULL,
        _status  TEXT NOT NULL,
        final_decision  TEXT NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );
