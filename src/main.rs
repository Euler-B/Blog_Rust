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
    use self::models::{Post, NewPost};
    use self::schema::posts::dsl::*;
    use self::schema::posts;

    let new_post = NewPost {
        title: "Go vs. Rust",
        body:  "Acompañanos en el siguiente articulo, donde aprenderemos los pros y contras de estos maravillosos lenguajes",
        slug:  "post-technical",
    };
    // Insertamos los registros
    let post: Post = diesel::insert_into(posts::table).values(new_post).get_result(&mut conn).expect("La insercion de datos a fallado");

    //Realizamos la consulta Select * from posts
    //let post_result = posts.load::<Post>(&mut conn).expect("Error al ejecutar la query");
    //Realizamos la consulta Select * from posts 1
    let post_result = posts.limit(1).load::<Post>(&mut conn).expect("Error al ejecutar la query");

    // Iteramos sobre la consulta
    for post in post_result {
        println!("{}", post.title);
    }
}
