# Todo

Cli simple de TODO escrita en Rust 

Compilar:
```
cargo build
```

Ejecutar:
```
cargo run
```
Estructura del proyecto

todo-cli/
├── Cargo.toml
└── src/
    ├── main.rs       # punto de entrada, parsea argumentos
    ├── todo.rs       # lógica: struct Task, operaciones
    └── storage.rs    # leer/guardar en archivo JSON