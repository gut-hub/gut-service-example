# jinx-service-example

Sample service to be used with jinx. (see [gut-jinx](https://github.com/gut-hub/gut-jinx))

### Docker secret
```shell
$ printf "Docker secret for jinx service" | docker secret create jinx_secret -
m69czl7ht2xi5ou2zao6zr5t3

# add to jinx.json
"image_secrets": ["jinx_secret:m69czl7ht2xi5ou2zao6zr5t3"],
```
