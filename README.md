# RUST-HTTP-LAMBDA

## Build and Deploy

[cargo-lambda](https://www.cargo-lambda.info/)
[prisma-client-rust](https://prisma.brendonovich.dev/)

Run cargo prisma generate to generate `/src/prisma.rs`

Add Trigger API Gateway with
```
/fn_name          -> function
/fn_name/{proxy+} -> function
```