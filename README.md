# 🎓 Student Management gRPC Demo

A comprehensive gRPC demo built with Rust and Tokio, featuring a student management system with separate server and client crates.

## 🏗️ Project Structure

```
rs-grpc-demo/
├── proto/              # Shared protocol buffer definitions
│   ├── proto/
│   │   └── student.proto
│   ├── build.rs
│   ├── Cargo.toml
│   └── src/lib.rs
├── server/             # gRPC server implementation
│   ├── Cargo.toml
│   └── src/main.rs
├── client/             # gRPC client demo
│   ├── Cargo.toml
│   └── src/main.rs
├── Cargo.toml          # Workspace configuration
├── run_demo.sh         # Demo script
└── README.md
```

## 🚀 Features

### Server Features
- **CRUD Operations**: Create, Read, Update, Delete students
- **Data Validation**: Validates student data (age, GPA, email, etc.)
- **In-Memory Storage**: Thread-safe HashMap with RwLock
- **Error Handling**: Proper gRPC status codes and error messages
- **Logging**: Console output for all operations

### Client Features
- **Complete Demo**: Demonstrates all CRUD operations
- **Sample Data**: Creates sample students automatically
- **Error Handling**: Graceful error handling and reporting
- **Interactive Output**: Clear, formatted console output

### Protocol Buffer Schema
- **Student Model**: ID, name, email, age, major, GPA
- **Service Methods**:
  - `CreateStudent` - Create a new student
  - `GetStudent` - Retrieve student by ID
  - `UpdateStudent` - Update existing student
  - `DeleteStudent` - Delete student by ID
  - `ListStudents` - List all students with pagination

## 🛠️ Prerequisites

- Rust 1.70+ with Cargo
- Protocol Buffers compiler (protoc) - usually installed automatically by tonic-build

## 🏃‍♂️ Quick Start

### Option 1: Use the Demo Script (Recommended)
```bash
./run_demo.sh
```

### Option 2: Manual Setup

1. **Build the project:**
```bash
cargo build --release
```

2. **Start the server (in one terminal):**
```bash
cargo run --bin server
```

3. **Run the client demo (in another terminal):**
```bash
cargo run --bin client
```

## 📋 Demo Output

The client demo will:
1. ✅ Create 3 sample students (Alice, Bob, Carol)
2. 📋 List all students
3. 🔍 Get a specific student by ID
4. 📝 Update a student's major and GPA
5. 📋 List students again (showing the update)
6. 🗑️ Delete a student
7. 📋 Final list (showing the deletion)

## 🔧 Development

### Adding New Fields
1. Update `proto/proto/student.proto`
2. Rebuild with `cargo build`
3. Update server validation logic
4. Update client demo as needed

### Testing Individual Operations
You can test individual operations by modifying the client code or using tools like `grpcurl`:

```bash
# Example: List students
grpcurl -plaintext localhost:50051 student.StudentService/ListStudents
```

## 📚 Learning Resources

This demo demonstrates:
- **Protocol Buffers**: Schema definition and code generation
- **Tonic**: Rust gRPC framework usage
- **Tokio**: Async runtime and concurrency
- **Error Handling**: gRPC status codes and Rust Result types
- **Data Validation**: Input validation and business logic
- **Workspace Management**: Multi-crate Rust projects

## 🤝 Contributing

Feel free to extend this demo with:
- Database persistence (PostgreSQL, SQLite)
- Authentication and authorization
- More complex queries and filtering
- Streaming operations
- TLS/SSL support
- Docker containerization
