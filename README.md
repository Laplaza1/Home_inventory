```markdown
# Home_inventory

**A full-stack home inventory management application**

Track household items, quantities, categories, prices, and recipes with a modern Rust backend and a JavaScript frontend. Built for personal or small household use, with multi-user support, authentication via cookies, rate limiting, and basic analytics/graph data endpoints.

> **Author**: Laplaza1 (Myself)
> **Repository**: [https://github.com/Laplaza1/Home_inventory](https://github.com/Laplaza1/Home_inventory)  
> **Status**: Active development 

---

## Overview

Home_inventory lets you manage the contents of your home (or multiple homes) in one place. You can add items with categories, quantities, units of measure, and unit prices; organize them by home; create recipes that reference inventory items; handle user registration/approval flows; and pull basic data for charts and dashboards.With agentic assistance provided in the management.

The **backend** is written in **Rust** using the Axum web framework and MongoDB. The **frontend** is a traditional JavaScript + HTML/CSS application that talks to the API.

### Key Features

- **Item Management**
  - Create, read, update, and delete inventory items
  - Fields: name, categories (array), quantity, method of measure, unit price (`Decimal128`), date, and associated home
  - Lookup of specific items by ID
  - Category-based counting (e.g., Meat, Spice, Vegetable, Fruit, Dairy, Cleaning, Animal)

- **User & Access Control**
  - User registration and login
  - Cookie-based sessions (`Session_ID`)
  - Pending user approval flow
  - Per-user home association and basic access levels (Creator / Admin / User)
  - User info (email, phone number, home)

- **Recipes**
  - Create recipes that reference simplified inventory items (name, quantity, unit)
  - Retrieve recipes

- **Data & Analytics**
  - Endpoints for pulling inventory data
  - Graph-specific data by ID
  - Admin/general data endpoints
  - Category statistics

- **Notifications**
  - Basic notification structure (SMS-style) with contact and message

- **Security & Infrastructure**
  - CORS configured for common local and GitHub Pages origins
  - Rate limiting (global + route-specific) via `axum-governor` and `lazy-limit`
  - Real IP extraction
  - Password hashing with SHA-256 (token generation)
  - Cookie handling with `axum-extra` / `cookie`

---

## Tech Stack

| Layer          | Technology                                      |
|----------------|-------------------------------------------------|
| Backend        | Rust                                            |
| Web Framework  | Axum 0.8 + Tokio                                |
| Database       | MongoDB (via `mongodb` crate + BSON)            |
| Auth / Cookies | Cookie-based sessions, SHA-256 tokens           |
| Rate Limiting  | axum-governor + lazy-limit                      |
| Frontend       | Vanilla JavaScript, HTML, CSS                   |
| Serialization  | Serde + serde_json                              |
| Other          | tower-http (CORS), reqwest, futures, chrono, etc.|

---

## Project Structure

----
Home_inventory/
├── Backend/                  # Rust Axum API
│   ├── src/
│   │   ├── main.rs           # Server setup, routes, CORS, rate limiting
│   │   └── routes.rs         # Handlers, models, business logic
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── README.md
├── Frontend/                 # Client-side application
│   ├── js/                   # JavaScript logic
│   └── static/               # HTML, CSS, assets
└── README.md                 # This file (root)
----

---

## Backend (Rust)

### Dependencies (from `Cargo.toml`)

- `axum` (with WebSocket and macros features)
- `axum-extra` (cookie support)
- `tokio` (full)
- `mongodb` + `bson`
- `serde` / `serde_json` / `serde_with`
- `tower-http` (CORS)
- `axum-governor`, `lazy-limit`, `real`
- `sha2`, `hex`, `base64`
- `chrono`, `futures`, `reqwest`, `anyhow`, etc.

### Main Data Models

- **Item** – `_id`, `item_name`, `category`, `quantity`, `method_measure`, `unit_price` , `date`, `home`
- **User** – `_id`, `username`, `password`, `status`, `title`, `token`
- **UseroInfo** – user_id, access level, home, email, phone_number
- **Pending** – username, email, home, password, phone_number, reason 
- **Recipe** – recipe_name + list of simplified items
- **Notification** – type, recipient contact, SMS message

### API Routes (high level)

| Method | Path                        | Description                      |
|--------|-----------------------------|----------------------------------|
| POST   | `/user`                     | Create user                      |
| GET    | `/user/{user_id}`           | Check / retrieve user            |
| PUT    | `/user`                     | Update user                      |
| DELETE | `/user/{user_id}`           | Delete user                      |
| POST   | `/pending`                  | Create pending registration      |
| POST   | `/login`                    | Login (sets session cookie)      |
| POST   | `/item`                     | Insert item                      |
| GET    | `/item`                     | Get items                        |
| GET    | `/specificItem/{item_id}`   | Get specific item                |
| PUT    | `/item`                     | Update item                      |
| DELETE | `/item`                     | Delete item                      |
| POST   | `/recipe`                   | Create recipe                    |
| GET    | `/recipe`                   | Get recipes                      |
| POST   | `/notify`                   | Send notification                |
| GET    | `/data`                     | Pull inventory data              |
| GET    | `/graph/{id}`               | Graph-specific data              |
| GET    | `/admin_data`               | General / admin data             |
| GET    | `/cookies`                  | Debug / show cookies             |
| GET    | `/test`                     | Test endpoint                    |

The server listens on `0.0.0.0:3000` by default.

### Rate Limiting

- Global default: 5 requests per second
- Special route example: 10 requests per second on `/api/special`
- Real IP layer is applied for accurate client identification

### CORS

Allowed origins include:
- `http://localhost:3000`
- `http://localhost`
- `http://127.0.0.1:5500`
- `https://laplaza1.github.io`

Credentials and common methods/headers are enabled.

---

## Getting Started

### Prerequisites

- Rust toolchain (rustup recommended) 
- MongoDB instance (local or remote)
- Node.js / file server for the frontend (static files)
>> Soon to need secret keys to foundation models

### 1. Clone the repository

```bash
git clone https://github.com/Laplaza1/Home_inventory.git
cd Home_inventory
```

### 2. Backend setup

```bash
cd Backend
```

Configure your MongoDB connection string (the code currently uses a database named `test`(Soon to be set in .env) and expects a properly configured client — update `handle_client()` or environment variables as needed).

Build and run:

```bash
cargo build
cargo run
```

The API will be available at `http://0.0.0.0:3000` (or `localhost:3000`).

### 3. Frontend

Serve the `Frontend` directory with any static file server (Live Server, `python -m http.server`, nginx, GitHub Pages, etc.). Point the frontend’s API base URL to your running Rust backend.

---

## Development Notes

- Session management relies on cookies (`Session_ID`). Many protected routes check for a valid session cookie.
- Token generation uses SHA-256 over combined values.
- Category statistics currently hard-code a list of common categories (Meat, Spice, Vegetable, Fruit, Dairy, Cleaning, Animal).
- Some recipe-related routes (specific get / delete) are commented out / still under development.
- Logging and timing (`Instant`) are used in several places for performance observation.
- The codebase is still evolving — expect some rough edges, especially around error handling and input validation.
- currently there is a TON of unwraps.

---

## Future / Possible Improvements

- Stronger password hashing (e.g., Argon2 / bcrypt) instead of plain SHA-256 for storage
- More complete recipe CRUD
- Reduce Structs
- Proper environment-based configuration (MongoDB URI, secrets, allowed origins)
- Input validation and better error responses
- JWT or more robust session management
- Docker / deployment configuration
- Expanded analytics and reporting
- Mobile-friendly frontend polish
- Agentic workflows for serviceable functions such as
    >>Checking Inventory for expired or near expired
    >>Making meals with current inventory
    >>Generating potential recipies 
    >>Making shopping list with nearby options with potential prices 

---




## Contributing

This is a personal project, feel free to open issues or pull requests if you want to collaborate or suggest improvements.



