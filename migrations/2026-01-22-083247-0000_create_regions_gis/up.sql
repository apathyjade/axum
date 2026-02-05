-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE regions (
    id SERIAL PRIMARY KEY,
    name TEXT,
    geom GEOMETRY(MULTIPOLYGON, 4326)  -- WGS84 坐标系
);