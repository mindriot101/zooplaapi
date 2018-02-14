--
-- PostgreSQL database dump
--

-- Dumped from database version 10.1
-- Dumped by pg_dump version 10.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

SET search_path = public, pg_catalog;

ALTER TABLE ONLY public.houses_locations DROP CONSTRAINT locations_house_id_fkey;
ALTER TABLE ONLY public.houses_urls DROP CONSTRAINT houses_urls_house_id_fkey;
ALTER TABLE ONLY public.houses_properties DROP CONSTRAINT houses_properties_house_id_fkey;
ALTER TABLE ONLY public.houses_agents DROP CONSTRAINT houses_agents_house_id_fkey;
ALTER TABLE ONLY public.houses_categories DROP CONSTRAINT house_categories_house_id_fkey;
ALTER TABLE ONLY public.houses_locations DROP CONSTRAINT locations_pkey;
ALTER TABLE ONLY public.houses_urls DROP CONSTRAINT houses_urls_pkey;
ALTER TABLE ONLY public.houses_properties DROP CONSTRAINT houses_properties_pkey;
ALTER TABLE ONLY public.houses DROP CONSTRAINT houses_pkey;
ALTER TABLE ONLY public.houses_agents DROP CONSTRAINT houses_agents_pkey;
ALTER TABLE ONLY public.houses_categories DROP CONSTRAINT house_categories_pkey;
ALTER TABLE ONLY public.__diesel_schema_migrations DROP CONSTRAINT __diesel_schema_migrations_pkey;
ALTER TABLE public.houses_urls ALTER COLUMN id DROP DEFAULT;
ALTER TABLE public.houses_properties ALTER COLUMN id DROP DEFAULT;
ALTER TABLE public.houses_locations ALTER COLUMN id DROP DEFAULT;
ALTER TABLE public.houses_categories ALTER COLUMN id DROP DEFAULT;
ALTER TABLE public.houses_agents ALTER COLUMN id DROP DEFAULT;
ALTER TABLE public.houses ALTER COLUMN id DROP DEFAULT;
DROP SEQUENCE public.locations_id_seq;
DROP SEQUENCE public.houses_urls_id_seq;
DROP TABLE public.houses_urls;
DROP SEQUENCE public.houses_properties_id_seq;
DROP TABLE public.houses_properties;
DROP TABLE public.houses_locations;
DROP SEQUENCE public.houses_id_seq;
DROP SEQUENCE public.houses_agents_id_seq;
DROP TABLE public.houses_agents;
DROP TABLE public.houses;
DROP SEQUENCE public.house_categories_id_seq;
DROP TABLE public.houses_categories;
DROP TABLE public.__diesel_schema_migrations;
DROP FUNCTION public.diesel_set_updated_at();
DROP FUNCTION public.diesel_manage_updated_at(_tbl regclass);
DROP EXTENSION plpgsql;
DROP SCHEMA public;
--
-- Name: public; Type: SCHEMA; Schema: -; Owner: simon
--

CREATE SCHEMA public;


ALTER SCHEMA public OWNER TO simon;

--
-- Name: SCHEMA public; Type: COMMENT; Schema: -; Owner: simon
--

COMMENT ON SCHEMA public IS 'standard public schema';


--
-- Name: plpgsql; Type: EXTENSION; Schema: -; Owner: 
--

CREATE EXTENSION IF NOT EXISTS plpgsql WITH SCHEMA pg_catalog;


--
-- Name: EXTENSION plpgsql; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION plpgsql IS 'PL/pgSQL procedural language';


SET search_path = public, pg_catalog;

--
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: simon
--

CREATE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


ALTER FUNCTION public.diesel_manage_updated_at(_tbl regclass) OWNER TO simon;

--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: simon
--

CREATE FUNCTION diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.diesel_set_updated_at() OWNER TO simon;

SET default_tablespace = '';

SET default_with_oids = false;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: simon
--

CREATE TABLE __diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE __diesel_schema_migrations OWNER TO simon;

--
-- Name: houses_categories; Type: TABLE; Schema: public; Owner: simon
--

CREATE TABLE houses_categories (
    id integer NOT NULL,
    property_type character varying(255) NOT NULL,
    category character varying(255) NOT NULL,
    price_modifier character varying(255),
    house_id integer
);


ALTER TABLE houses_categories OWNER TO simon;

--
-- Name: house_categories_id_seq; Type: SEQUENCE; Schema: public; Owner: simon
--

CREATE SEQUENCE house_categories_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE house_categories_id_seq OWNER TO simon;

--
-- Name: house_categories_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: simon
--

ALTER SEQUENCE house_categories_id_seq OWNED BY houses_categories.id;


--
-- Name: houses; Type: TABLE; Schema: public; Owner: simon
--

CREATE TABLE houses (
    id integer NOT NULL,
    price integer NOT NULL,
    first_published_date character varying(32) NOT NULL,
    last_published_date character varying(32) NOT NULL,
    listing_id integer DEFAULT '-1'::integer NOT NULL
);


ALTER TABLE houses OWNER TO simon;

--
-- Name: houses_agents; Type: TABLE; Schema: public; Owner: simon
--

CREATE TABLE houses_agents (
    id integer NOT NULL,
    name character varying(255) NOT NULL,
    phone character varying(255) NOT NULL,
    house_id integer
);


ALTER TABLE houses_agents OWNER TO simon;

--
-- Name: houses_agents_id_seq; Type: SEQUENCE; Schema: public; Owner: simon
--

CREATE SEQUENCE houses_agents_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE houses_agents_id_seq OWNER TO simon;

--
-- Name: houses_agents_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: simon
--

ALTER SEQUENCE houses_agents_id_seq OWNED BY houses_agents.id;


--
-- Name: houses_id_seq; Type: SEQUENCE; Schema: public; Owner: simon
--

CREATE SEQUENCE houses_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE houses_id_seq OWNER TO simon;

--
-- Name: houses_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: simon
--

ALTER SEQUENCE houses_id_seq OWNED BY houses.id;


--
-- Name: houses_locations; Type: TABLE; Schema: public; Owner: simon
--

CREATE TABLE houses_locations (
    id integer NOT NULL,
    longitude real NOT NULL,
    latitude real NOT NULL,
    street_name character varying(255) NOT NULL,
    displayable_address character varying(255) NOT NULL,
    house_id integer
);


ALTER TABLE houses_locations OWNER TO simon;

--
-- Name: houses_properties; Type: TABLE; Schema: public; Owner: simon
--

CREATE TABLE houses_properties (
    id integer NOT NULL,
    description text NOT NULL,
    num_bathrooms integer NOT NULL,
    num_bedrooms integer NOT NULL,
    num_floors integer NOT NULL,
    house_id integer
);


ALTER TABLE houses_properties OWNER TO simon;

--
-- Name: houses_properties_id_seq; Type: SEQUENCE; Schema: public; Owner: simon
--

CREATE SEQUENCE houses_properties_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE houses_properties_id_seq OWNER TO simon;

--
-- Name: houses_properties_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: simon
--

ALTER SEQUENCE houses_properties_id_seq OWNED BY houses_properties.id;


--
-- Name: houses_urls; Type: TABLE; Schema: public; Owner: simon
--

CREATE TABLE houses_urls (
    id integer NOT NULL,
    details character varying(255) NOT NULL,
    property_report character varying(255) NOT NULL,
    house_id integer
);


ALTER TABLE houses_urls OWNER TO simon;

--
-- Name: houses_urls_id_seq; Type: SEQUENCE; Schema: public; Owner: simon
--

CREATE SEQUENCE houses_urls_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE houses_urls_id_seq OWNER TO simon;

--
-- Name: houses_urls_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: simon
--

ALTER SEQUENCE houses_urls_id_seq OWNED BY houses_urls.id;


--
-- Name: locations_id_seq; Type: SEQUENCE; Schema: public; Owner: simon
--

CREATE SEQUENCE locations_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE locations_id_seq OWNER TO simon;

--
-- Name: locations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: simon
--

ALTER SEQUENCE locations_id_seq OWNED BY houses_locations.id;


--
-- Name: houses id; Type: DEFAULT; Schema: public; Owner: simon
--

ALTER TABLE ONLY houses ALTER COLUMN id SET DEFAULT nextval('houses_id_seq'::regclass);


--
-- Name: houses_agents id; Type: DEFAULT; Schema: public; Owner: simon
--

ALTER TABLE ONLY houses_agents ALTER COLUMN id SET DEFAULT nextval('houses_agents_id_seq'::regclass);


--
-- Name: houses_categories id; Type: DEFAULT; Schema: public; Owner: simon
--

ALTER TABLE ONLY houses_categories ALTER COLUMN id SET DEFAULT nextval('house_categories_id_seq'::regclass);


--
-- Name: houses_locations id; Type: DEFAULT; Schema: public; Owner: simon
--

ALTER TABLE ONLY houses_locations ALTER COLUMN id SET DEFAULT nextval('locations_id_seq'::regclass);


--
-- Name: houses_properties id; Type: DEFAULT; Schema: public; Owner: simon
--

ALTER TABLE ONLY houses_properties ALTER COLUMN id SET DEFAULT nextval('houses_properties_id_seq'::regclass);


--
-- Name: houses_urls id; Type: DEFAULT; Schema: public; Owner: simon
--

ALTER TABLE ONLY houses_urls ALTER COLUMN id SET DEFAULT nextval('houses_urls_id_seq'::regclass);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: simon
--

ALTER TABLE ONLY __diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: houses_categories house_categories_pkey; Type: CONSTRAINT; Schema: public; Owner: simon
--

ALTER TABLE ONLY houses_categories
    ADD CONSTRAINT house_categories_pkey PRIMARY KEY (id);


--
-- Name: houses_agents houses_agents_pkey; Type: CONSTRAINT; Schema: public; Owner: simon
--

ALTER TABLE ONLY houses_agents
    ADD CONSTRAINT houses_agents_pkey PRIMARY KEY (id);


--
-- Name: houses houses_pkey; Type: CONSTRAINT; Schema: public; Owner: simon
--

ALTER TABLE ONLY houses
    ADD CONSTRAINT houses_pkey PRIMARY KEY (id);


--
-- Name: houses_properties houses_properties_pkey; Type: CONSTRAINT; Schema: public; Owner: simon
--

ALTER TABLE ONLY houses_properties
    ADD CONSTRAINT houses_properties_pkey PRIMARY KEY (id);


--
-- Name: houses_urls houses_urls_pkey; Type: CONSTRAINT; Schema: public; Owner: simon
--

ALTER TABLE ONLY houses_urls
    ADD CONSTRAINT houses_urls_pkey PRIMARY KEY (id);


--
-- Name: houses_locations locations_pkey; Type: CONSTRAINT; Schema: public; Owner: simon
--

ALTER TABLE ONLY houses_locations
    ADD CONSTRAINT locations_pkey PRIMARY KEY (id);


--
-- Name: houses_categories house_categories_house_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: simon
--

ALTER TABLE ONLY houses_categories
    ADD CONSTRAINT house_categories_house_id_fkey FOREIGN KEY (house_id) REFERENCES houses(id);


--
-- Name: houses_agents houses_agents_house_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: simon
--

ALTER TABLE ONLY houses_agents
    ADD CONSTRAINT houses_agents_house_id_fkey FOREIGN KEY (house_id) REFERENCES houses(id);


--
-- Name: houses_properties houses_properties_house_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: simon
--

ALTER TABLE ONLY houses_properties
    ADD CONSTRAINT houses_properties_house_id_fkey FOREIGN KEY (house_id) REFERENCES houses(id);


--
-- Name: houses_urls houses_urls_house_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: simon
--

ALTER TABLE ONLY houses_urls
    ADD CONSTRAINT houses_urls_house_id_fkey FOREIGN KEY (house_id) REFERENCES houses(id);


--
-- Name: houses_locations locations_house_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: simon
--

ALTER TABLE ONLY houses_locations
    ADD CONSTRAINT locations_house_id_fkey FOREIGN KEY (house_id) REFERENCES houses(id);


--
-- Name: public; Type: ACL; Schema: -; Owner: simon
--

GRANT ALL ON SCHEMA public TO PUBLIC;


--
-- PostgreSQL database dump complete
--

