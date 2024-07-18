# api-diff

To compares the response bodies and headers of the two APIs and displays the diff.

## Usage

```sh
$ ./target/release/api-diff -h
Usage: api-diff [OPTIONS] --endpoint-a <ENDPOINT_A> --endpoint-b <ENDPOINT_B>

Options:
  -a, --endpoint-a <ENDPOINT_A>      First API endpoint
  -b, --endpoint-b <ENDPOINT_B>      Second API endpoint
      --show-headers                 Show headers diff
  -H, --headers-file <HEADERS_FILE>  Request headers file (JSON or YAML)
  -q, --query-file <QUERY_FILE>      Query parameters file (JSON or YAML)
  -h, --help                         Print help (se
```

## Compare responses of two APIs and display diff

```diff
$ ./target/release/api-diff -a https://jsonplaceholder.typicode.com/comments/1 -b https://jsonplaceholder.typicode.com/comments/2
 {
-  "body": "laudantium enim quasi est quidem magnam voluptate ipsam eos\ntempora quo necessitatibus\ndolor quam autem quasi\nreiciendis et nam sapiente accusantium",
-  "email": "Eliseo@gardner.biz",
-  "id": 1,
-  "name": "id labore ex et quam laborum",
+  "body": "est natus enim nihil est dolore omnis voluptatem numquam\net omnis occaecati quod ullam at\nvoluptatem error expedita pariatur\nnihil sint nostrum voluptatem reiciendis et",
+  "email": "Jayne_Kuhic@sydney.com",
+  "id": 2,
+  "name": "quo vero reiciendis velit similique earum",
   "postId": 1
 }
```

If you wanna header diff, specified `-H` option

```sh
$ ./target/release/api-diff -a <ENDPOINT1> -b <ENDPOINT2> -H
```

## add custom headers and query parameters

headers.json

```json
{
  "headers": {
    "Authorization": "Bearer your_token",
    "Content-Type": "application/json",
    "X-Api-Key": "dummy-api-key"
  }
}
```

query.json

```json
{
  "query": {
    "id": "1"
  }
}
```

```sh
$ ./target/release/api-diff  -a https://jsonplaceholder.typicode.com/posts  -b https://jsonplaceholder.typicode.com/posts -q ./src/query.json -H ./src/headers.json
----------------------
Request for endpoint_a:
URL: https://jsonplaceholder.typicode.com/posts?id=1
Headers: {"authorization": "Bearer your_token", "x-api-key": "dummy-api-key", "content-type": "application/json"}
----------------------
Request for endpoint_b:
URL: https://jsonplaceholder.typicode.com/posts?id=1
Headers: {"authorization": "Bearer your_token", "x-api-key": "dummy-api-key", "content-type": "application/json"}
 [
   {
     "body": "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto",
     "id": 1,
     "title": "sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
     "userId": 1
   }
 ]
```

## LICENSE

Apache License Version 2.0

## Author

Mao Nabeta(@nabetama)
