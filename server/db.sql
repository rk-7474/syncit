CREATE DATABASE IF NOT EXISTS syncit;

USE syncit;

CREATE TABLE drawers (
    data VARCHAR(255),
    name VARCHAR(255)
);

CREATE TABLE files (
    id VARCHAR(255),
    buffer MEDIUMBLOB
);