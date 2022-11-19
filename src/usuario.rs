use std::io::Result;
use actix_web::{web, Responder, post, get, put, delete};
use self::models::*;
use diesel::prelude::*;
use crate::*;
use self::schema::usuarios::dsl::*;
use crate::schema::usuarios;

#[post("/usuarios")]
pub async fn cadastrar_usuario(usuario_form: web::Json<UsuarioForm>) -> Result<impl Responder> {
  let connection = &mut establish_connection();

  diesel::insert_into(usuarios::table)
    .values(&*usuario_form)
    .get_result::<Usuario>(connection)
    .expect("Error saving new post");

  Ok(web::Json(usuario_form))
}

#[put("/usuarios/{id_usuario}")]
pub async fn editar_usuario(id_usuario: web::Path<i32>, usuario_form: web::Json<UsuarioForm>) -> Result<impl Responder> {
  let connection = &mut establish_connection();

  let campos_update = (
    nome.eq(&*usuario_form.nome),
    email.eq(&*usuario_form.email),
    senha.eq(&*usuario_form.senha)
  );

  diesel::update(usuarios)
    .filter(id.eq(&*id_usuario))
    .set(campos_update)
    .execute(connection)
    .expect("Erro na atualização");

  Ok(web::Json(usuario_form))
}

#[delete("/usuarios/{id_usuario}")]
pub async fn excluir_usuario(id_usuario: web::Path<i32>) -> Result<impl Responder> {
  let connection = &mut establish_connection();

  diesel::delete(usuarios.filter(id.eq(&*id_usuario)))
    .execute(connection)
    .expect("Error on delete");

  Ok(web::Json("Usuário excluido com sucesso.".to_string()))
}

#[get("/usuarios/{id_usuario}")]
pub async fn listar_usuario(id_usuario: web::Path<i32>) -> Result<impl Responder> {
  let connection = &mut establish_connection();

  let usuario = usuarios
    .filter(id.eq(&*id_usuario))
    .first::<Usuario>(connection)
    .expect("Error on delete");

  Ok(web::Json(usuario))
}

#[get("/usuarios")]
pub async fn listar_usuarios() -> Result<impl Responder> {
  let connection = &mut establish_connection();

  let results: Vec<Usuario> = usuarios
    .load::<Usuario>(connection)
    .expect("Error loading posts");

  Ok(web::Json(results))
}