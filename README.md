# Chat-FacApp

A simple desktop chat application built with **Rust**, **Rocket** backend, and **Tauri** for desktop integration.  
This project allows users to create or join chat rooms and communicate in real-time.

---

## 🔧 Features

- Real-time chat rooms with multiple users  
- Lightweight and fast backend powered by Rust & Rocket  
- Desktop application via Tauri (runs on Windows, macOS, Linux)  
- Easy to extend for new features (like file sharing or notifications)  

---

## 🚀 Getting Started

### Prerequisites

- Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)  
- Node.js & npm (for Tauri frontend)  
- Tauri prerequisites: [https://tauri.app/v1/guides/getting-started/prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Installation

1. Clone the repository:

```bash
git clone https://github.com/Macaron-s1/chat-facapp
cd chat-facapp
```

2.Install frontend dependencies:

```bash
cd src-tauri
npm install
```

3.Run the application in development mode:

```bash
cargo tauri dev
```

The desktop app should open, allowing you to create or join chat rooms.

👨‍💻 Author

Iancu Aurelian (GitHub: Aurasj)
Created as a learning project to explore Rust, Rocket, and Tauri desktop apps.

🔓 Usage / License

This project is free to use, modify, and share.
Feel free to clone this repository for learning, personal projects, or as a starting point for your own chat application.
