
# Full-Stack To-Do App (Rust + Next.js)

> Welcome! This is my personal project: a full-stack To-Do List application built with a Rust backend and a Next.js (React) frontend. The goal is to provide a fast, modern, and reliable to-do app while learning and demonstrating full-stack development with Rust and JavaScript/TypeScript.

---

## 🚀 Project Overview

This app lets you manage your daily tasks efficiently. The backend is written in Rust (using Actix-web), providing a high-performance REST API. The frontend is built with Next.js and React for a smooth, interactive user experience.

**Main Features:**
- Add, edit, and delete to-do items
- Mark tasks as complete/incomplete
- Persistent storage (database)
- Responsive, modern UI

---

## 🛠️ Tech Stack

- **Backend:** Rust, Actix-web, Diesel ORM
- **Frontend:** Next.js, React
- **Database:** (Configure your preferred DB in `rust/Cargo.toml` and backend code)
- **API:** RESTful endpoints

---

## 📁 Project Structure

```
rust/           # Rust backend (API server)
	src/
		main.rs     # Rust entry point
	Cargo.toml    # Rust dependencies

pages/          # Next.js frontend pages
Components/     # React components (frontend)
public/         # Static assets
styles/         # CSS
```

---

## 🏁 Getting Started

### 1. Backend (Rust)

```
cd rust
# Install dependencies
cargo build
# Run the server
cargo run
```
The backend will start on the configured port (default: 8000). Adjust settings in `rust/src/main.rs` as needed.

### 2. Frontend (Next.js)

```
# In the project root
npm install
npm run dev
```
The frontend will start on [http://localhost:3000](http://localhost:3000).

---

## ⚙️ Configuration

- Update backend API URL in the frontend if needed (see API calls in `pages/` or `Components/`).
- Set up your database connection in the Rust backend (`rust/Cargo.toml` and code).

---

## 🙋‍♂️ Author

This project was created by me as a learning and portfolio piece. Ofcourse, with some help from existing projects. Feel free to use, modify, or contribute!

---

## 📄 License

MIT License. See `LICENSE` for details.
