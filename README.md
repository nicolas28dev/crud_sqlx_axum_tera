# CRUD_SQLX_AXUM_TERA

This project is a basic CRUD (Create, Read, Update, Delete) Multi-Page Application (MPA) implemented in the Rust programming language. Key components include:

- **sqlx crate**: Provides connectivity to a PostgreSQL database.
- **axum crate**: Serves as the web server.
- **tera crate**: Generates HTML on the server.

If Rust is not installed on your device, first follow the official instructions: https://doc.rust-lang.org/book/ch01-01-installation.html

## Database Requirements

This project requires an existing PostgreSQL database with a table named `Author`.

### Table Schema

| Column Name | Type  |
|-------------|-------|
| id          | UUID  |
| first_name  | TEXT  |
| last_name   | TEXT  |

### Setup Script

```sql
CREATE TABLE IF NOT EXISTS public."Author"
(
    "id" uuid NOT NULL,
    "first_name" text COLLATE pg_catalog."default",
    "last_name" text COLLATE pg_catalog."default",
    CONSTRAINT "Author_pkey" PRIMARY KEY ("id")
);
```

#### Setting up the Environment Variable

The project should have the following structure:

src/  
|-- controllers/  
|-- custom_helpers/  
|-- db_actions/  
|-- models/  
|-- main.rs  

Create a .env file at the root of src directory.
    
 src/  
|-- controllers/  
|-- custom_helpers/  
|-- db_actions/  
|-- models/  
|-- main.rs  
|-- .env  
    
Inside the .env file, add the following content (use your own values):

```
DATABASE_URL=postgres://username:password@localhost/database_name
```
