CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS todo (
  id uuid NOT NULL DEFAULT uuid_generate_v4(),
  title VARCHAR(255) NOT NULL,
  checked boolean NOT NULL DEFAULT false,
  create_time timestamp with time zone NOT NULL DEFAULT NOW(),
  modify_time timestamp with time zone NOT NULL DEFAULT NOW()
);
