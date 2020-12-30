-- Your SQL goes here
CREATE TABLE public.rsvp
(
    id SERIAL PRIMARY KEY,
    "marktId" integer,
    "marktDate" date,
    "erkenningsNummer" character varying(255),
    attending boolean,
    "createdAt" timestamp with time zone NOT NULL DEFAULT NOW(),
    "updatedAt" timestamp with time zone NOT NULL DEFAULT NOW()
)