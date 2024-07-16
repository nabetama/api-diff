# api-diff

To compares the response bodies and headers of the two APIs and displays the diff.

## Usage

```sh
$ ./target/release/api-diff --help
Compare two API endpoints

Usage: api-diff [OPTIONS] --endpoint1 <ENDPOINT1> --endpoint2 <ENDPOINT2>

Options:
  -a, --endpoint1 <ENDPOINT1>
          First API endpoint

  -b, --endpoint2 <ENDPOINT2>
          Second API endpoint

  -H, --show-headers
          Show header diffs

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
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

## LICENSE

Apache License Version 2.0

## Author

Mao Nabeta(@nabetama)
