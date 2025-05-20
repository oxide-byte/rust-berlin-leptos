CREATE DATABASE keycloak;
CREATE USER keycloak WITH PASSWORD 'keycloak';
GRANT ALL ON DATABASE keycloak TO keycloak;

-- Keycloak

\connect keycloak keycloak;
CREATE SCHEMA keycloak;