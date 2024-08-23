# AWS Lambda: Usando Cargo Lambda

Constuyendo una lambda sencilla con [cargo lambda](http://www.cargo-lambda.info).
La lambda lista el conjunto de Buckets que se encuentran creados en el AWS S3.

Para ejecutar la lambda localmente:

```bash
cargo lambda watch

```

Para invocar la lambda:

```bash
cargo lambda invoke --data-ascii "{\"command\": \"hi\"}"
```
