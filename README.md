**NX Actix_Web Boilerplate**

## **Introduce**  
This is an **Actix Web** based backend service in the **Nx** monorepo, which uses **SQLx** as the Database interface, **Redis** as the cache, and **RabbitMQ** for messaging.  

## **Starting**  

### **1. Cloning the Repository**  
```bash
    git clone {this repo url} {your_project_directory_name}
```
and then reinitialize .git file 

```bash
    rm -rf .git
    git init
```

### **2. Update Project.json and Cargo.toml**
For integrate the services to nx monorepo 

**project.json**
```json
    ...
    "sourceRoot": "apps/{your_project_name}/src"....

    ..."options": {
          "target-dir": "dist/target/{your_project_name}"
        }

```

**cargo.toml**
```toml
    [package]
    name = "{your_project_name}"
    version = "0.1.0"
    edition = "2021"
    repository = "{your_project_repository}"

```

### **3. Running the Service**
Run the Actix Web in nx monorepos:
```bash
    nx run {apps_names}:run
```
or run it on the app directly:

```bash 
    cargo run
```

### Features
* 🚀 Actix Web - Fast Rust-based web framework.
* 🛢 SQLx - A runtime-less ORM that supports PostgreSQL, MySQL, and SQLite.
* ✅ Pre-commit Hooks (Husky) - Standardise code before commit.
* 📜 Commit Lint - Standardise the commit format to make it neater.
