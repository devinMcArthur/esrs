## Start Eventstore Locally

```shell
docker run --name esdb-node -it -p 2113:2113 \
    eventstore/eventstore:latest --insecure --run-projections=All \ 
    --enable-atom-pub-over-http
```
