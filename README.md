# Kobink

Automatically sync local books to your Kobo by pretending to be the Kobo Store

### Setup

On the Kobo, open `/.kobo/Kobo/Kobo eReader.conf` and change `api_endpoint` to your local machine where Kobink runs (e.g., `http://192.168.0.1:3000`).

Install dependencies and run:

```
cargo run
```