-- Create the enumeration type
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE user_role AS ENUM ('admin', 'user');

-- Create the roles table
CREATE TABLE
    roles (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        name user_role NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT NOW ()
    );

-- Create the users table
CREATE TABLE
    users (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        username TEXT NOT NULL UNIQUE,
        password_hash TEXT NOT NULL,
        email TEXT NOT NULL UNIQUE,
        user_role user_role NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW (),
        updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW () 
    );