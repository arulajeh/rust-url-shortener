# URL Shortener Service

This is a URL Shortener service built with Rust using Actix Web for the backend and PostgreSQL as the database. It allows users to generate short URLs and resolve them back to their original URLs.

---

## Features
- Generate short URLs for any given URL.
- Resolve short URLs back to their original URLs.
- Persistent storage using PostgreSQL.
- Optimized with a hash index for faster lookups.
- Profiling-ready with support for performance optimization.

---

## Prerequisites

### Dependencies:
1. **Rust**: Ensure Rust is installed. Use [Rustup](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **PostgreSQL**: Install PostgreSQL and ensure it is running.
3. **Node.js**: (Optional) For tools like `k6` to stress test the API.

### Required Rust Crates:
Ensure the following dependencies are added in `Cargo.toml`:
- `actix-web`
- `sqlx`
- `chrono`
- `dotenvy`
- `serde`
- `rand`

---

## Database Setup

1. Create the database schema:
   ```sql
   CREATE TABLE public.urls (
       id bigserial NOT NULL,
       shorten varchar NOT NULL,
       url varchar NOT NULL,
       created_at timestamp NULL,
       counter int8 NULL,
       CONSTRAINT urls_pk PRIMARY KEY (id)
   );
   CREATE INDEX urls_shorten_hash_idx ON urls USING hash (shorten);
   ```

2. Configure the `.env` file:
   ```env
   DATABASE_URL=postgres://username:password@localhost:5432/yourdatabase
   SERVER_PORT=3000
   ```
   Replace `username`, `password`, and `yourdatabase` with your PostgreSQL credentials.

---

## Running the Project

1. Clone the repository:
   ```bash
   git clone <repository_url>
   cd <repository_name>
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the server:
   ```bash
   cargo run
   ```

4. The server will start at `http://0.0.0.0:3000` (default port).

---

## API Endpoints

### 1. **Generate Short URL**
**POST** `/short/generate`

**Request Body:**
```json
{
    "url": "https://example.com"
}
```

**Response:**
```json
{
    "message": "URL shortened successfully",
    "code": "SUCCESS201",
    "data": {
        "short": "Ab3Xy9Zq"
    }
}
```

### 2. **Redirect to Original URL**
**POST** `/short/redirect`

**Request Body:**
```json
{
    "short": "Ab3Xy9Zq"
}

**Response (Success):**
```json
{
    "message": "Redirect URL found",
    "code": "SUCCESS200",
    "data": {
        "original_url": "https://example.com"
    }
}
```

**Response (Failure):**
```json
{
    "message": "Shortened URL not found",
    "code": "ERROR404",
    "data": null
}
```

---

## Profiling and Testing

### 1. Profiling with `cargo flamegraph`:
- Install flamegraph:
  ```bash
  cargo install flamegraph
  sudo apt install linux-perf
  ```
- Generate a flamegraph:
  ```bash
  sudo cargo flamegraph
  ```
- Open the generated `flamegraph.svg` in your browser.

### 2. Stress Testing with `k6`:
- Install `k6`:
  ```bash
  brew install k6  # macOS
  sudo apt install k6  # Ubuntu
  ```
- Create a test script `test.js`:
  ```javascript
  import http from 'k6/http';
  import { sleep } from 'k6';

  export default function () {
      http.post('http://localhost:3000/short/generate', JSON.stringify({
          url: 'https://example.com'
      }), {
          headers: { 'Content-Type': 'application/json' },
      });
      sleep(1);
  }
  ```
- Run the test:
  ```bash
  k6 run test.js
  ```

---

## Future Enhancements
- Add caching for frequently accessed URLs.
- Implement rate-limiting to prevent abuse.
- Add analytics for URL usage.
- Support custom short URL paths.

---

## License
This project is licensed under the MIT License. See the LICENSE file for details.

