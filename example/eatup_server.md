# 🚀 EatUp Server API

Here you will find the documentation of the server microservice of the EatUp project

## 📡 Product: `/api/v1`

All the information about the products available.

<details><summary> <b>GET <code>/api/v1/products</code></b></summary>

Fetch the products

### Filters

| Name | Example | Description |
| ---- | --- | --- |
| `category` | `?...&category=XXXXXX...` | The category to include in the response. It can be used multiple times to fetch all the selected categories |
| `allergy` | `?...&allergy=XXXXXX...` | Excludes all the products that contains any of the specified allergies. It can be used multiple times. |


<details><summary>Examples</summary>

### Example fetching all the products

```javascript
fetch("/api/v1/api/v1/products", {
  method: "GET",
})
.then(response => response.json()) // If valid
//.then(response => response.text()) // If invalid
.then(json => console.log(json));
.catch(error => console.error(error));
```

Response:

This is the result

```json
200

[
  {
    id: "XXXXXX",
    name: "YYYYYY",
    description: "ZZZZZZ",
    img_id: "YYYYYYY.png",
    price: 42.0,
    allergies: [
      {
        id: "AAAAAAA",
        name: "BBBBBBB",
        img_id: "CCCCCCC.png"
      }
      ...
    ],
    categories: [
      {
        id: "DDDDDDD",
        name: "EEEEEEE"
      }
      ...
    ]
  },
  ...
]
```

### Get all starters lactose free

```javascript
fetch("/api/v1/api/v1/products?category=EEEEEEE&allergy=BBBBBB", {
  method: "GET",
})
.then(response => response.json()) // If valid
//.then(response => response.text()) // If invalid
.then(json => console.log(json));
.catch(error => console.error(error));
```

Response:

If XXXXXX is the category for starters and YYYYYY the allergy lactose; this are the products:

```json
200

[
  {
    id: "XXXXXX",
    name: "YYYYYY",
    description: "ZZZZZZ",
    img_id: "YYYYYYY.png",
    price: 42.0,
    allergies: [
      {
        id: "AAAAAAA",
        name: "BBBBBBB",
        img_id: "CCCCCCC.png"
      }
      ...
    ],
    categories: [
      {
        id: "DDDDDDD",
        name: "EEEEEEE"
      }
      ...
    ]
  },
  ...
]
```

</details>

</details>

<br/>

## 📡 Session: `/api/v1`

The moment when the client is actively using the app in a table

<details><summary> <b>GET <code>/api/v1/sessions</code></b></summary>

Fetch all the sessions

### Filters

| Name | Example | Description |
| ---- | --- | --- |
| `in_progress` | `?...&in_progress=true/false...` | Filter only the active or inactive sessions |
| `table_id` | `?...&table_id=XXXXXX...` | Filter the sessions done in a specific table |


<details><summary>Examples</summary>

### Get all the sessions

```javascript
fetch("/api/v1/api/v1/sessions", {
  method: "GET",
})
.then(response => response.json()) // If valid
//.then(response => response.text()) // If invalid
.then(json => console.log(json));
.catch(error => console.error(error));
```

Response:

This is the result

```json
200

[
  {
    id: "XXXXXX",
    table_id: "YYYYYY",
    in_progress: true/false
  },
  ...
]
```

### Get all active sessions

```javascript
fetch("/api/v1/api/v1/sessions?in_progress=true", {
  method: "GET",
})
.then(response => response.json()) // If valid
//.then(response => response.text()) // If invalid
.then(json => console.log(json));
.catch(error => console.error(error));
```

Response:

This is the result

```json
200

[
  {
    id: "XXXXXX",
    table_id: "YYYYYY",
    in_progress: true/false
  },
  ...
]
```

### Get all sessions done in table `YYYYYY`

```javascript
fetch("/api/v1/api/v1/sessions?table_id=YYYYYY", {
  method: "GET",
})
.then(response => response.json()) // If valid
//.then(response => response.text()) // If invalid
.then(json => console.log(json));
.catch(error => console.error(error));
```

Response:

This is the result

```json
200

[
  {
    id: "XXXXXX",
    table_id: "YYYYYY",
    in_progress: true/false
  },
  ...
]
```

</details>

</details>

<details><summary> <b>GET <code>/api/v1/session_id/:simple_id</code></b></summary>

Fetch the session_id with the simple_id

### Parameters

| Name | Example | Description |
| ---- | --- | --- |
| `simple_id` | `.../XXXXXX YYYYYY ZZZZZZ/...` | The simple_id of the session |


<details><summary>Examples</summary>

### Get the session_id of the session with simple_id `XXXXXX YYYYYY ZZZZZZ`

```javascript
fetch("/api/v1/api/v1/session_id/XXXXXX YYYYYY ZZZZZZ", {
  method: "GET",
})
.then(response => response.json()) // If valid
//.then(response => response.text()) // If invalid
.then(json => console.log(json));
.catch(error => console.error(error));
```

Response:

This is the result

```json
200

{
    "simple_id": "XXXXXX YYYYYY ZZZZZZ",
    "id": "AAAAAAA",
    "qr_img": "/qr/AAAAAAA.png"
}
```

</details>

</details>

<details><summary> <b>POST <code>/api/v1/session/:table_id</code></b></summary>

Create a new session

### Parameters

| Name | Example | Description |
| ---- | --- | --- |
| `table_id` | `.../XXXXXX/...` | The id of the table where the session should be created |


<details><summary>Examples</summary>

### Create a new session in table `BBBBBBB`

```javascript
fetch("/api/v1/api/v1/session/BBBBBBB", {
  method: "POST",
})
.then(response => response.json()) // If valid
//.then(response => response.text()) // If invalid
.then(json => console.log(json));
.catch(error => console.error(error));
```

Response:

This is the result

```json
200

{
    "simple_id": "XXXXXX YYYYYY ZZZZZZ",
    "id": "AAAAAAA",
    "qr_img": "/qr/AAAAAAA.png"
}
```

### Attempt to create a new session in table `BBBBBBB`

```javascript
fetch("/api/v1/api/v1/session/BBBBBBB", {
  method: "POST",
})
.then(response => response.json()) // If valid
//.then(response => response.text()) // If invalid
.then(json => console.log(json));
.catch(error => console.error(error));
```

Response:

This is the result

```json
409

There is already a session in progress for table mesa
```

</details>

</details>

<details><summary> <b>PATCH <code>/api/v1/session/:session_id/end</code></b></summary>

End a session

### Parameters

| Name | Example | Description |
| ---- | --- | --- |
| `session_id` | `.../XXXXXX/...` | The id of the session to end |


<details><summary>Examples</summary>

### Ending a session with session_id `AAAAAAA`

```javascript
fetch("/api/v1/api/v1/session/AAAAAAA/end", {
  method: "PATCH",
})
.then(response => response.json()) // If valid
//.then(response => response.text()) // If invalid
.then(json => console.log(json));
.catch(error => console.error(error));
```

Response:

This is the result

```json
200

"Session ended"
```

</details>

</details>

<br/>

## 📡 Orders: `/orders`

Logic to manage the orders made by the clients in a session.

<details><summary> <b>GET <code>/orders/session/:session_id</code></b></summary>

Fetch all the orders of a session

### Parameters

| Name | Example | Description |
| ---- | --- | --- |
| `session_id` | `.../XXXXXX/...` | The id of the session |


<details><summary>Examples</summary>

### Get all the orders of the session with id `AAAAAAA`

```javascript
fetch("/orders/api/v1/orders/session/AAAAAAA", {
  method: "GET",
})
.then(response => response.json()) // If valid
//.then(response => response.text()) // If invalid
.then(json => console.log(json));
.catch(error => console.error(error));
```

Response:

This is the result

```json
200

[
  {
    "id": "AAAAAAA",
    "products": [
      {
        "id": "BBBBBBB",
        "quantity": 2,
        "product": {
          "id": "CCCCCCC",
          "name": "Bruschetta",
          "description": "Tomato, garlic, basil, olive oil",
          "img_id": "bruchetta.png",
          "price": 5.0,
          "allergies": [
            {
              "id": "DDDDDDD",
              "name": "Gluten",
              "img_id": "gluten.png"
            },
            {
              "id": "EEEEEEE",
              "name": "Lactose",
              "img_id": "lactose.png"
            }
          ],
          "categories": [
            {
              "id": "FFFFFFF",
              "name": "Appetizers"
            }
          ]
        }
      },
      ...
    ],
  },
  ...
]
```

</details>

</details>

<details><summary> <b>POST <code>/orders/session/:session_id</code></b></summary>

Create a new order in a session

### Parameters

| Name | Example | Description |
| ---- | --- | --- |
| `session_id` | `.../XXXXXX/...` | The id of the session |


<details><summary>Examples</summary>

### Create a new order in the session with id `AAAAAAA`

```javascript
fetch("/orders/api/v1/orders/session/AAAAAAA", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify(
  {
      "products": [
        {
          "quantity": 3
          "product": {
            "id": "BBBBBBB"
            "name": "Bruschetta"
            "description": "Tomato, garlic, basil, olive oil"
            "img_id": "bruchetta.png"
            "price": 5.0
            "allergies": [
              {
                "id": "DDDDDDD"
                "name": "Gluten"
                "img_id": "gluten.png"
              },
              {
                "id": "EEEEEEE"
                "name": "Lactose"
                "img_id": "lactose.png"
              }
            ],
            "categories": [
              {
                "id": "FFFFFFF"
                "name": "Appetizers"
              }
            ]
          }
        },
        ...
      ]
    }
  )
})
.then(response => response.json()) // If valid
//.then(response => response.text()) // If invalid
.then(json => console.log(json));
.catch(error => console.error(error));
```

Response:

This is the result

```json
200

"Order created"
```

### Create a new order in invalid session with id `BBBBBBB`

```javascript
fetch("/orders/api/v1/orders/session/BBBBBBB", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify(
  {
      "products": [
        {
          "quantity": 3
          "product": {
            "id": "BBBBBBB"
            "name": "Bruschetta"
            "description": "Tomato, garlic, basil, olive oil"
            "img_id": "bruchetta.png"
            "price": 5.0
            "allergies": [
              {
                "id": "DDDDDDD"
                "name": "Gluten"
                "img_id": "gluten.png"
              },
              {
                "id": "EEEEEEE"
                "name": "Lactose"
                "img_id": "lactose.png"
              }
            ],
            "categories": [
              {
                "id": "FFFFFFF"
                "name": "Appetizers"
              }
            ]
          }
        },
        ...
      ]
    }
  )
})
.then(response => response.json()) // If valid
//.then(response => response.text()) // If invalid
.then(json => console.log(json));
.catch(error => console.error(error));
```

Response:

This is the result

```json
409

Invalid session id
```

</details>

</details>

<br/>

## 📡 Special requests: `/`

This section contains the documentation for the special requests

<details><summary> <b>GET <code>/</code></b></summary>

Ping request to check if the server is up

<details><summary>Examples</summary>

### Ping the server

```javascript
fetch("/", {
  method: "GET",
})
.then(response => response.json()) // If valid
//.then(response => response.text()) // If invalid
.then(json => console.log(json));
.catch(error => console.error(error));
```

Response:

This is the result

```json
200

"Eatup up and running!"
```

</details>

</details>

<details><summary> <b>GET <code>/:file_route</code></b></summary>

Allows to fetch all files in the public directory of the installation volume.

### Parameters

| Name | Example | Description |
| ---- | --- | --- |
| `file_route` | `.../qr/AAAAAAA.png/...` | The route of the file |


<details><summary>Examples</summary>

### Get QR png

```javascript
fetch("//qr/AAAAAAA.png", {
  method: "GET",
})
.then(response => response.json()) // If valid
//.then(response => response.text()) // If invalid
.then(json => console.log(json));
.catch(error => console.error(error));
```

Response:

This is the result

```json
200

**PNG file**
```

### Get qr that does not exist

```javascript
fetch("//qr/BBBBBBB.png", {
  method: "GET",
})
.then(response => response.json()) // If valid
//.then(response => response.text()) // If invalid
.then(json => console.log(json));
.catch(error => console.error(error));
```

Response:

This is the result

```json
404

"QR not found. Are you sure the QR should be valid?"
```

</details>

</details>

<br/>

<details><summary><h2>🔧 Details</h2></summary>

## Legend

This are the meanings of the symbols used in this document

| Element | Meaning |
| ------- | ------- |
| `:session_id` | The id of the session |
| `:table_id` | The id of the table |
| `true/false` | A boolean value. It must be `true` or `false` |
| `XXXXXX` | Some value that would be replaced for something else in the real situation. |
| `...` | More parameters can be added to the request |


## API Codes

| Code | Meaning | Description |
| ---- | ------- | ----------- |
| `200` | *200 OK* | The request was successful. |
| `404` | *404 Not Found* | The resource does not exist. |
| `409` | *409 Conflict* | Something is not right with the request. |
| `500` | *500 Internal Server Error* | Something went wrong on the server. Please contact with the administrator. |
| `501` | *501 Not Implemented* | The endpoint is not implemented yet. |


</details>

<br/>

Made with ❤️ using [api_docs_generator](https://github.com/Jkutkut/rust-api_docs_generator) v0.7.0.
