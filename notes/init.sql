-- Database initialization script for note-api
-- This script runs when the PostgreSQL container starts for the first time

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create database if it doesn't exist (this is handled by POSTGRES_DB env var)
-- The database 'note_api' will be created automatically

-- Set timezone
SET timezone = 'UTC';

-- Create a simple health check function
CREATE OR REPLACE FUNCTION health_check()
RETURNS TEXT AS $$
BEGIN
    RETURN 'Database is healthy';
END;
$$ LANGUAGE plpgsql;

-- Log initialization
DO $$
BEGIN
    RAISE NOTICE 'Database initialization completed successfully';
END $$;