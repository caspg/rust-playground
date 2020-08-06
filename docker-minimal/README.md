Build and run:

```bash
$ docker build -t slim-rust .
$ docker run --rm -it slim-rust
```

Find image size:

```bash
$ docker images | grep slim-rust
slim-rust                                                 latest              cbe3d4b21c0e        9 seconds ago       8.75MB
```
