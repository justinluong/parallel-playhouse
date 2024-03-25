Creates truncated versions of a set of files in parallel

```find . -name "*.json" | parallel 'head -n 2024 {} > truncated/{.}.json'```

