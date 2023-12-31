-- Add migration script here
CREATE TABLE customers (
    id UUID PRIMARY KEY,
    first_name VARCHAR(253) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    aadhar_number BIGINT NOT NULL,
    date_of_birth VARCHAR(255) NOT NULL,
    gender VARCHAR(10) NOT NULL,
    address TEXT NOT NULL,
    is_active BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE dependents (
    id UUID PRIMARY KEY,
    customer_id UUID REFERENCES customers(id),
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    aadhar_number BIGINT  NOT NULL,
    date_of_birth VARCHAR(10) NOT NULL,
    gender VARCHAR(10) NOT NULL,
    address TEXT NOT NULL,
    relation VARCHAR(255) NOT NULL,
    relationship VARCHAR(20) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

