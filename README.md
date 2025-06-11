# LinkLair: A Modern URL Shortener API

LinkLair is a URL shortener API built with Rust, Axum, and SQLx. This project is designed to be a portfolio piece showcasing modern backend development practices, including iterative feature development, API design, authentication, and database management.

## About The Project

The core idea is to build a robust API starting from a simple Minimum Viable Product (MVP) and progressively adding features in logical phases. This mirrors real-world software development, starting with a solid foundation and expanding with user accounts, analytics, and more complex functionality.

### Tech Stack

* **Language:** [Rust](https://www.rust-lang.org/)
* **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
* **Async Runtime:** [Tokio](https://tokio.rs/)
* **Database ORM/Driver:** [SQLx](https://github.com/launchbadge/sqlx)
* **Database:** [SQLite](https://www.sqlite.org/index.html) (for simplicity, easily upgradable)
* **Validation:** [validator](https://crates.io/crates/validator)
* **Authentication:** JWT (JSON Web Tokens)
* **Password Hashing:** (To be added in Phase 2)

## Features

### Phase 1: Core MVP (Complete)

* [x] Create a short URL from a long URL.
* [x] Redirect a short URL to its original destination using a `308 Permanent Redirect`.
* [x] Validate incoming URLs.
* [x] Graceful JSON error handling.

### Phase 2: User Accounts & Ownership (In Progress)

* [ ] User registration and login endpoints.
* [ ] JWT-based authentication for protected routes.
* [ ] Associate links with the users who create them.
* [ ] Endpoint for users to view their own links.

### Future Phases (Planned)

* **Phase 3: Analytics & Tracking:** Track the number of clicks for each link.
* **Phase 4: "Pro" Features:** Custom short URLs, QR code generation, link previews.
* **Phase 5: Architectural Evolution:** Discussion on scaling, upgrading to PostgreSQL/Redis, and moving to microservices.

## Getting Started

Follow these instructions to get a local copy up and running for development and testing.

### Prerequisites

* **Rust Toolchain:** Install from [rustup.rs](https://rustup.rs/).
* **sqlx-cli:** This is required to manage database migrations.
    ```sh
    cargo install sqlx-cli
    ```

### Installation & Setup

1.  **Clone the repository:**
    ```sh
    git clone git@github.com:ishworii/URL_shortner.git
    cd URL_shortner 
    ```

2.  **Create the environment file:**
    Create a `.env` file in the root of the project and add the database URL.
    ```sh
    echo 'DATABASE_URL="sqlite:links.db"' > .env
    ```

3.  **Run the database migrations:**
    This will create the `links.db` file and set up all the necessary tables.
    ```sh
    sqlx migrate run
    ```

### Running the Application

1.  **Run the server:**
    ```sh
    cargo run
    ```
    The server will start on `http://127.0.0.1:8000`.

2.  **Run in watch mode (optional but recommended for development):**
    Install `cargo-watch` and use it to automatically recompile on file changes.
    ```sh
    cargo install cargo-watch
    cargo watch -x run
    ```

## API Endpoints

### `POST /api/links`

Creates a new short link.

* **Body:**
    ```json
    {
      "url": "[https://a-very-long-url.com/with/lots/of/stuff](https://a-very-long-url.com/with/lots/of/stuff)"
    }
    ```
* **Success Response (200 OK):**
    ```json
    {
      "short_url": "http://localhost:8000/aB1cDeF"
    }
    ```
* **Error Response (400 Bad Request):**
    ```json
    {
      "error": "Input validation failed: url: must be a valid URL"
    }
    ```

### `GET /{short_code}`

Redirects to the original URL.

* **Example Request:** `GET /aB1cDeF`
* **Success Response:** `308 Permanent Redirect` with a `location` header pointing to the original URL.
* **Error Response (404 Not Found):**
    ```json
    {
      "error": "Resource not found"
    }
    ```
