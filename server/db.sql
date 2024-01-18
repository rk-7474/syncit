CREATE DATABASE IF NOT EXISTS syncit;

-- Use the "syncit" database
USE syncit;

-- Create the "drawers" table
CREATE TABLE drawers (
    data VARCHAR(255),
    name VARCHAR(255)
);

-- Create the "files" table
CREATE TABLE files (
    id VARCHAR(255),
    buffer MEDIUMBLOB
);