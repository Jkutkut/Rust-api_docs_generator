# API Docs Generator:
This rust program allows to generate a markdown file with the documentation of an API by parsing a JSON file.

## Usage:

### Run directly:
- Go to the directory where you have the `api.json` file.

```bash
docker run -it --rm -v $(pwd):/app -w /app jkutkut/api_docs_generator api.json
```

### Run and rename output file:
- Go to the directory where you have the `api.json` file.

```bash
docker run -it --rm -v $(pwd):/app -w /app jkutkut/api_docs_generator api.json output.md
```

# Special thanks:
- Original template: [azagniotov's template](https://gist.github.com/azagniotov/a4b16faf0febd12efbc6c3d7370383a6)