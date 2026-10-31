# WASM Integration

The `itp-wasm` crate provides WebAssembly bindings for using ITP in the browser.

## Building

```bash
cd src/itp-wasm
wasm-pack build --target web
```

This generates a `pkg/` directory with JavaScript/TypeScript bindings.

## API

All functions accept and return JSON strings:

### grep

```javascript
import { grep } from 'itp-wasm';

const options = JSON.stringify({ case_sensitive: false });
const result = grep("TODO", fileContent, options);
console.log(JSON.parse(result));
// { matches: [...], count: 3 }
```

### sed

```javascript
import { sed } from 'itp-wasm';

const options = JSON.stringify({});
const result = sed("s/old/new/g", fileContent, options);
console.log(JSON.parse(result));
// { output: "...", changes: [...] }
```

### awk

```javascript
import { awk } from 'itp-wasm';

const options = JSON.stringify({ field_separator: ",", has_header: true });
const result = awk(csvContent, options);
console.log(JSON.parse(result));
```

### aql

```javascript
import { aql } from 'itp-wasm';

const result = aql('find "TODO" | count', fileContent);
console.log(JSON.parse(result));
```

### semantic_search

```javascript
import { semantic_search } from 'itp-wasm';

const docs = JSON.stringify([
  { file: "a.rs", line: 1, content: "Rust programming" }
]);
const result = semantic_search("systems programming", docs, 5);
console.log(JSON.parse(result));
```

### ontology

```javascript
import { ontology } from 'itp-wasm';

const ont = JSON.parse(ontology());
console.log(ont.tool);        // "itp"
console.log(ont.capabilities); // [...]
```

### version

```javascript
import { version } from 'itp-wasm';
console.log(version()); // "0.1.0"
```

## Error Handling

All functions return `{"error": "message"}` on failure instead of throwing:

```javascript
const result = JSON.parse(grep("[invalid", content, "{}"));
if (result.error) {
  console.error("Failed:", result.error);
}
```
