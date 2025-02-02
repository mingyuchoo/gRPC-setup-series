# Calculator

## How to build this project

```bash
$ cargo build
```

## How to run this project

Run server as below

```bash
$ cargo run --bin client
```

Run client as below

```bash
$ cargo run --bin client
```

Test server as below

```bash
$ grpcurl -plaintext -proto ./proto/calculator.proto -d '{"a": 2, "b": 3}' '[::1]:50051' calculator.Calculator.Add
$ grpcurl -emit-defaults -plaintext '[::1]:50051' calculator.Admin.GetRequestCount
```

## References

- https://youtu.be/kerKXChDmsE?si=7bGzfmBtfNycppie
- https://github.com/hyperium/tonic
- https://github.com/dreamsofcode-io/grpcalculator-web
- https://github.com/dreamsofcode-io/grpcalculator-rs
