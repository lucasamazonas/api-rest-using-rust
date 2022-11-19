use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::usuarios;

#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct Usuario {
  pub id: i32,
  pub nome: String,
  pub email: String,
  pub senha: String,
}

#[derive(Insertable)]
#[derive(Serialize, Deserialize)]
#[diesel(table_name = usuarios)]
pub struct UsuarioForm {
  pub nome: String,
  pub email: String,
  pub senha: String,
}