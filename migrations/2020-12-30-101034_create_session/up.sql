-- Your SQL goes here
CREATE TABLE public.session
(
    sid character varying(255) NOT NULL,
    sess json,
    expire timestamp with time zone,
    CONSTRAINT session_pkey PRIMARY KEY (sid)
)