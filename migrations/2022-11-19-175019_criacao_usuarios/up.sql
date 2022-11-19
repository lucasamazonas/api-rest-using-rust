-- Your SQL goes here
CREATE TABLE usuarios (
  id SERIAL NOT NULL,
  nome VARCHAR(50) NOT NULL,
  email VARCHAR(100) NOT NULL,
  senha VARCHAR(32) NOT NULL,
  CONSTRAINT usuarios_pk PRIMARY KEY (id)
);