-- Your SQL goes here
CREATE TABLE markets (
  "id" SERIAL PRIMARY KEY,
  "name" VARCHAR NOT NULL,
  "abbreviation" VARCHAR NOT NULL,
  "createdAt" timestamp with time zone NOT NULL DEFAULT NOW(),
  "updatedAt" timestamp with time zone NOT NULL DEFAULT NOW()
)