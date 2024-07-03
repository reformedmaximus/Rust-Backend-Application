# Rust-Backend-Application
This project is a Rust backend application using Axum, Tokio, and PostgreSQL, featuring `GET` and `POST` endpoints for creating and retrieving custom details with JSON payload validation.

## Requirements

- [Git](https://git-scm.com/)
- [Rustup](https://rustup.rs/)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [PostgreSQL](https://www.postgresql.org/)

## Project Structure

- `src/`: Contains the main application code.
  - `main.rs`: The main entry point of the application, setting up routes and initializing the server.
  - `db.rs`: Contains the database connection setup.
  - `handlers.rs`: Contains the route handler functions for the endpoints.
  - `models.rs`: Defines the data structures used in the application.
  - `.env`: Contains environment variables for database connection.
- `Cargo.toml`: Specifies the dependencies and project metadata.


## Setup Instructions

1. Install Rust
   
Please check the official Rust installation guide before you start. [Official Documentation](https://www.rust-lang.org/tools/install). Also, make sure you have installed the latest version of Rust.

2. Clone the Repository
```bash
$ git clone https://github.com/your-username/your-repo.git
$ cd your-repo
```

3. Install the dependencies included in the Cargo.toml file with : 
```bash
cargo build
```
    
### Configure Environment Variables
Create a .env file in the root of your project and add your database URL in this format :
```bash
DATABASE_URL=postgres://your-username:your-password@localhost/database_name
```


### Running the Application
```bash
cargo run
```

### How to test the Endpoints
To test the functionality of the endpoints, you can use the following curl commands:

- **POST Request:**
  - **Description:** Adds a new custom detail to the database.
  - **Command:**
    ```sh
    curl -X POST http://localhost:3000/custom_details -H "Content-Type: application/json" -d "{\"name\": \"Rusty Ocean11\", \"email\": \"iloverust@gmail.com\"}"
    ```
  - **Explanation:**
    - `-X POST`: Specifies the request method as POST.
    - `-H "Content-Type: application/json"`: Sets the header to indicate that the content type is JSON.
    - `-d "{\"name\": \"Rusty Ocean11\", \"email\": \"iloverust@gmail.com\"}"`: Provides the JSON payload with the name and email to be added.

- **GET Request:**
  - **Description:** Retrieves all custom details from the database.
  - **Command:**
    ```sh
    curl http://localhost:3000/custom_details
    ```
  - **Explanation:**
    - This command performs a GET request to the specified URL to fetch the list of custom details stored in the database.











