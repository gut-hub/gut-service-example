# jinx-service-example

Sample service to be used with jinx. (see [gut-jinx](https://github.com/gut-hub/gut-jinx))

### Docker secret
```shell
$ printf "Docker secret for jinx service" | docker secret create jinx_secret -
m69czl7ht2xi5ou2zao6zr5t3

# add to jinx.json
"image_secrets": ["jinx_secret:m69czl7ht2xi5ou2zao6zr5t3"],
```

### Test service
```shell
$ docker build -t jinx-service-example .
$ docker run -d --rm -p 80:3000 jinx-service-example
$ curl localhost
```
