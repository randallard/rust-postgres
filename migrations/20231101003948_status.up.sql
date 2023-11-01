-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE service_state AS ENUM (
    'AVAILABLE', 'NOT_AVAILABLE'
);

CREATE TABLE 
    service_statuses (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        ip TEXT NOT NULL,
        state service_state NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW (),
        updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW ()         
    );
