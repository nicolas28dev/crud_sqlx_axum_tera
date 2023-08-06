use crate::{
    custom_helpers::uuid_helper::str_to_uuid, 
    models::author_form::AuthorForm,
    db_actions::authors_db_actions,
    TEMPLATES};
use sqlx::PgPool;
use tera::Context;
use axum::{
    extract,
    extract::Form,
    extract::State,
    response::Html,
};

pub async fn show_all_authors(State(pool): State<PgPool>) -> Html<String>  {
    let authors_selected = match authors_db_actions::select_all_authors(&pool).await {
        Ok(authors) => {
            println!("Selected: {:?}", authors);
            authors
        },
        Err(err) => {
            eprintln!("Error selecting authors: {:?}", err);
        return Html("<h1>Failed to find authors</h1>".to_string())
        }
    };

    let mut context = tera::Context::new();
    context.insert("authors", &authors_selected);

    match TEMPLATES.render("authors.html", &context) {
        Ok(html) =>{
            return Html(html)
        }
        Err(e) => {
            println!("Parsing error(s): {}", e);
            return Html("<h1>Error : rendering failed</h1>".to_string())
        } 
    }
}

pub async fn show_author(State(pool): State<PgPool>, extract::Path(guid): extract::Path<String>) -> Html<String>  {
    let author_id = match str_to_uuid(&guid){
        Ok(uuid) => {
            uuid
        }
        Err(err) => {
            eprintln!("Error casting uuid: {:?}", err);
            return Html("<h1>Failed to handle the provided id</h1>".to_string())
        }
    };

    let author_selected = match authors_db_actions::select_author(&pool, &author_id).await {
        Ok(author) => {
            println!("Selected: {:?}", author);
            author
        },
        Err(err) => {
            eprintln!("Error selecting author: {:?}", err);
        return Html("<h1>Failed to find the author</h1>".to_string())
        }
    };

    let mut context = Context::new();
    context.insert("author", &author_selected);
    
    match TEMPLATES.render("author_display.html", &context) {
        Ok(html) =>{
            return Html(html)
        }
        Err(e) => {
            println!("Parsing error(s): {}", e);
            return Html("<h1>Error : rendering failed</h1>".to_string())
        } 
    }

} 

pub async fn show_author_form() -> Html<String>  {
    let context = Context::new();
    
    Html(TEMPLATES.render("author_form.html", &context).unwrap())
}

pub async fn accept_author_form(State(pool): State<PgPool>, Form(input): Form<AuthorForm>) -> Html<String> {
    match authors_db_actions::insert_author(&pool, &input).await {
    Ok(author) => {
        println!("Inserted: {:?}", author);
    },
    Err(err) => eprintln!("Error inserting author: {:?}", err)
    }
    println!("Inserted: {:?}", &input);

    show_all_authors(State(pool)).await
}

pub async fn update_author_form(State(pool): State<PgPool>, extract::Path(guid): extract::Path<String>, Form(input): Form<AuthorForm>) -> Html<String> {
    let author_id = match str_to_uuid(&guid){
        Ok(uuid) => {
            uuid
        }
        Err(err) => {
            eprintln!("Error casting uuid: {:?}", err);
            return Html("<h1>Failed to handle the provided id</h1>".to_string())
        }
    };

    match authors_db_actions::update_author(&pool, &author_id, &input).await {
    Ok(author) => {
        println!("Updated: {:?}", author);
    },
    Err(err) => eprintln!("Error updating author: {:?}", err)
    }
    println!("Updated: {:?}", &input);

    show_all_authors(State(pool)).await
}

pub async fn delete_author(State(pool): State<PgPool>, extract::Path(guid): extract::Path<String>) -> Html<String>  {
    let author_id = match str_to_uuid(&guid){
        Ok(uuid) => {
            uuid
        }
        Err(err) => {
            eprintln!("Error casting uuid: {:?}", err);
            return Html("<h1>Failed to handle the provided id</h1>".to_string())
        }
    };
    
    match authors_db_actions::delete_author(&pool, &author_id).await {
        Ok(_) => {
            println!("Deleted: {:?}", &guid);
        },
        Err(err) => eprintln!("Error deleting author: {:?}", err)
        }
        println!("Deleted: {:?}", &guid);

        show_all_authors(State(pool)).await
}