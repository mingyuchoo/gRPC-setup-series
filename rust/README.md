# README

## How to create a gRPC project with Tonic

### Create a project

```bash
$ cargo new {project-name}
$ cd {project-name}
```

### Add essential dependencies

```bash
$ cargo add prost
$ cargo add tonic
$ cargo add --build tonic-build
```

### Create and implement `proto/{proto_name}.proto` file

This is an example

```proto
# greeter.proto
syntax = "proto3";
package greeter;

service Greeting {
    rpc SayHello (HelloRequest) returns (HelloResponse);
}

message HelloRequest {
   string name = 1;
}

message HelloResponse {
    string message = 1;
}
```

### Create and implement `build.rs` file

This is an example

```rust
// build.rs
fn main() {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_protos(&["proto/greeter.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
```

### Build project to generate code

If you build the project, you can get an source code.

```bash
$ cargo build
```
