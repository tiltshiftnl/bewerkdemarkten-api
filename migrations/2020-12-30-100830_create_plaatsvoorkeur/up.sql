-- Your SQL goes here
CREATE TABLE public.plaatsvoorkeur
(
    id SERIAL PRIMARY KEY,
    "marktId" integer,
    "erkenningsNummer" character varying(255),
    "plaatsId" character varying(255),
    priority integer,
    "createdAt" timestamp with time zone NOT NULL DEFAULT NOW(),
    "updatedAt" timestamp with time zone NOT NULL DEFAULT NOW(),
    CONSTRAINT "plaatsvoorkeur_marktId_erkenningsNummer_plaatsId_key" UNIQUE ("marktId", "erkenningsNummer", "plaatsId")
)