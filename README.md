# ซอร์สโค้ดนี้ ใช้สำหรับเป็นตัวอย่างเท่านั้น ถ้านำไปใช้งานจริง ผู้ใช้ต้องจัดการเรื่องความปลอดภัย และ ประสิทธิภาพด้วยตัวเอง

# Person Information Web Application

A simple web application built with Rust, Actix-web, and Tera templates that allows users to submit and view personal information.

## Features

- User-friendly form for submitting personal information
- Display of submitted information in a formatted view
- Responsive design with clean CSS styling
- Server-side form processing
- Template-based rendering

## Prerequisites

- Rust (latest stable version)
- Cargo package manager

## Installation

1. Clone the repository:

TODO:
/////////////

````bashgit clone <repository-url>
cd person_app```
2. Build the project:
```bashcargo build --release
````

## Running the Application

1. Start the server:`bash
cargo run`
2. Open your web browser and navigate to:

```http://localhost:8080

```

## Project Structure

````person_app/
├── src/│   └── main.rs         # Main application code
├── templates/│   ├── index.html      # Form template
│   └── result.html     # Result display template├── static/
│   └── style.css       # CSS styling├── Cargo.toml          # Project dependencies
└── README.md           # This file```
## Dependencies
- actix-web (4.0): Web framework
- actix-files (0.6): Static file serving- tera (1.17): Template engine
- serde: Serialization framework- serde_json: JSON support
## Contributing
1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)5. Open a Pull Request
## License

This project is licensed under the MIT License - see the LICENSE file for details.






































````
