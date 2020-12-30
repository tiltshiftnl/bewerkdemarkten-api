-- Your SQL goes here
CREATE TABLE public.log
(
    id SERIAL PRIMARY KEY,
    ts timestamp with time zone NOT NULL DEFAULT NOW(),
    level character varying(255) NOT NULL DEFAULT 'debug'::character varying,
    msg text NOT NULL,
    meta json
)