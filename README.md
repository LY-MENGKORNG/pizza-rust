# Pizza App (Rust) — REST API

Simple JSON API built with **Axum**.

## Run

```bash
cargo run
```

By default it listens on `0.0.0.0:3000`. Override with `PORT`:

```bash
PORT=8080 cargo run
```

## Endpoints

- `GET /health` → `{ "status": "ok" }`
- `GET /pizzas` → list pizzas
- `POST /pizzas` → create a pizza
- `GET /pizzas/:id` → fetch a pizza
- `PATCH /pizzas/:id` → update a pizza
- `DELETE /pizzas/:id` → delete a pizza

## Example curl

Create:

```bash
curl -s -X POST http://localhost:3000/pizzas ^
  -H "content-type: application/json" ^
  -d "{\"name\":\"Margherita\",\"size\":\"medium\",\"toppings\":[\"basil\"]}"
```

List:

```bash
curl -s http://localhost:3000/pizzas
```


