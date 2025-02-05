--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2
-- Dumped by pg_dump version 17.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_managerid integer NOT NULL,
    dname character(50) NOT NULL,
    dno integer NOT NULL,
    dlocation character(50),
    pno integer NOT NULL
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_managerid, dname, dno, dlocation, pno) FROM stdin;
108	Administration                                    	1	Ikeja                                             	44
101	Account                                           	2	Egbeda                                            	11
100	Packaging                                         	3	Ajah                                              	44
120	Research                                          	4	VI                                                	33
97	Account                                           	5	Magodo                                            	22
122	Operations                                        	6	Mile 2                                            	44
107	Packaging                                         	7	Ketu                                              	55
\.


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dept_managerid);


--
-- PostgreSQL database dump complete
--

