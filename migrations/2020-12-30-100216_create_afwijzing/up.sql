-- Your SQL goes here
CREATE TABLE public.afwijzing
(
    id SERIAL PRIMARY KEY,
    "marktId" integer,
    "marktDate" date,
    "reasonCode" integer,
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
    CONSTRAINT "afwijzing_marktId_marktDate_reasonCode_key" UNIQUE ("marktId", "marktDate", "reasonCode")
)
