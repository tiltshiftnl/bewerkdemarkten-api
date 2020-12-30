-- Your SQL goes here
CREATE TABLE public.allocation
(
    id SERIAL PRIMARY KEY,
    "marktId" integer,
    "marktDate" date,
    "plaatsId" character varying(255),
    "erkenningsNummer" character varying(255),
    plaatsvoorkeuren text[],
    anywhere boolean,
    minimum integer,
    maximum integer,
    bak boolean,
    "eigenMaterieel" boolean,
    "brancheId" integer,
    "createdAt" timestamp with time zone NOT NULL DEFAULT NOW(),
    "updatedAt" timestamp with time zone NOT NULL DEFAULT NOW(),
    CONSTRAINT "allocation_marktId_marktDate_plaatsId_key" UNIQUE ("marktId", "marktDate", "plaatsId")
)