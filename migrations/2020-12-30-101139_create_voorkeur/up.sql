-- Your SQL goes here
CREATE TABLE public.voorkeur
(
    id SERIAL PRIMARY KEY,
    "erkenningsNummer" character varying(255) NOT NULL,
    "marktId" integer,
    "marktDate" date,
    anywhere boolean,
    minimum integer,
    maximum integer,
    "brancheId" character varying(255),
    "parentBrancheId" character varying(255),
    inrichting character varying(255),
    "absentFrom" date,
    "absentUntil" date,
    "createdAt" timestamp with time zone NOT NULL DEFAULT NOW(),
    "updatedAt" timestamp with time zone NOT NULL DEFAULT NOW(),
    CONSTRAINT "voorkeur_marktId_marktDate_key" UNIQUE ("marktId", "marktDate")
)