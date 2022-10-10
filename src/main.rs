#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenvy::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::models::PostSimplificado;

fn main() {
    // Lectura de variable de entorno
    dotenv().ok();
    let db_url     = env::var("DATABASE_URL").expect("db url variable no encontrada");  
    
    // Conexion a la base de datos 
    let mut conn = PgConnection::establish(&db_url).expect("No nos pudimos conectar a la base de datos");
    
    // Indicamos que vamos a usar el esquema Post y el modelo
    use self::models::{Post, NewPost, PostSimplificado};
    use self::schema::posts::dsl::*;
    use self::schema::posts;

    //Bloque de codigo usado para insertar datos en nuestra db//
    //let new_post = NewPost {
        //title: "Rust y sus aplicaiones en el ecosistema blockchain",
        //body:  "Acompañanos en el siguiente articulo, donde aprenderemos las bondades de este lenguaje a la hora de crear aplicaciones enfocadas en la blockchain",
        //slug:  "post-technical",
    //};
    // Insertamos los registros
    //let post: Post = diesel::insert_into(posts::table).values(new_post).get_result(&mut conn).expect("La insercion de datos a fallado");
    

    //Bloque de codigo usado para la consulta de datos en nuestra db//
    //Realizamos la consulta Select * from posts
    println!("*********************************");
    println!("*      Query sin limites        *");
    println!("*********************************");
    let post_result = posts.load::<Post>(&mut conn).expect("Error al ejecutar la query");
    // Iteramos sobre la consulta
    for post in post_result {
        println!("{:?}", post);
    }

    //Realizamos la consulta Select * from posts 1
    println!("*********************************");
    println!("*      Query con limites        *");
    println!("*********************************");
    let post_result = posts.limit(1).load::<Post>(&mut conn).expect("Error al ejecutar la query");

    // Iteramos sobre la consulta
    for post in post_result {
        println!("{:?}", post);
    }

    //Realizamos la consulta Select title, body from posts 1
    println!("**********************************");
    println!("* Query con columnas especificas *");
    println!("**********************************");
    let post_result = posts.select((title, body)).limit(1).load::<PostSimplificado>(&mut conn).expect("Error al ejecutar la query");

    // Iteramos sobre la consulta
    for post in post_result {
        println!("{:?}", post);
    }

    //Realizamos la consulta Select * from posts order by id limit 1
    println!("*************************************");
    println!("* Query con limites ordenado por id *");
    println!("*************************************");
    let post_result = posts.order(id).limit(1).load::<Post>(&mut conn).expect("Error al ejecutar la query");
    // Iteramos sobre la consulta
    for post in post_result {
        println!("{:?}", post);
    }

    //Realizamos la consulta Select * from posts order by id limit 1
    println!("**********************************************************");
    println!("* Query con limites ordenado por id de manera descendente*");
    println!("**********************************************************");
    let post_result = posts.order(id.desc()).limit(1).load::<Post>(&mut conn).expect("Error al ejecutar la query");
    // Iteramos sobre la consulta
    for post in post_result {
        println!("{:?}", post);
    }

    //Realizamos la consulta Select title, body from posts where id = 2 limit 1
    println!("*********************************");
    println!("*      Query con where          *");
    println!("*********************************");
    let post_result = posts.filter(id.eq(2)).limit(1).load::<Post>(&mut conn).expect("Error al ejecutar la query");

    // Iteramos sobre la consulta
    for post in post_result {
        println!("{:?}", post);
    }

    //Realizamos la consulta Select title, body from posts where slug = post-technical limit 1
    println!("*********************************");
    println!("*      Query con where          *");
    println!("*********************************");
    let post_result = posts.filter(slug.eq("post-technical")).limit(1).load::<Post>(&mut conn).expect("Error al ejecutar la query");

    // Iteramos sobre la consulta
    for post in post_result {
        println!("{:?}", post);
    }
}
