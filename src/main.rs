#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenvy::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

fn main() {
    // Lectura de variable de entorno
    dotenv().ok();
    let db_url     = env::var("DATABASE_URL").expect("db url variable no encontrada");  
    
    // Conexion a la base de datos 
    let mut conn = PgConnection::establish(&db_url).expect("No nos pudimos conectar a la base de datos");
    
    // Indicamos que vamos a usar el esquema Post y el modelo
    use self::models::Post;
    use self::schema::posts::dsl::*;

    //Realizamos la consulta Select * from posts
    let post_result = posts.load::<Post>(&mut conn).expect("Error al ejecutar la query");

    // Iteramos sobre la consulta
    for post in post_result {
        println!("{}", post.title);
    }
}
