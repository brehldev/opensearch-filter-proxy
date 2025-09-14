# Opensearch Filter Proxy 

OpenSearch Filter Proxy is a lightweight proxy server that automatically applies global filters to OpenSearch queries.

It works by intercepting requests to an OpenSearch instance, injecting predefined filters into the query, and forwarding the modified request to the OpenSearch server. These filters can be used to enforce data access rules based on user roles, permissions, or other criteria.

This tool is especially useful when the OpenSearch security plugin isn’t sufficient—for example, when more complex filtering is required than what JWT roles can provide. 

⚠️ Keep in mind: the effectiveness of this proxy depends entirely on the filters you configure.

## Installation

You can launch the project, with opensearch using docker compose

```bash
docker compose up -d
```
### Adding Sample Data
To load sample data into your OpenSearch instance:

```bash
cd ./data 

curl -X POST "http://localhost:9200/_bulk" \
  -H "Content-Type: application/x-ndjson" \
  --data-binary "@bulk.ndjson"

```

## Environment Variables

The proxy can be configured using the following environment variables:

| Variable                         | Default                | Description                                                                |
|----------------------------------|------------------------|----------------------------------------------------------------------------|
| `REVERSE_PROXY_TARGET_URL`       | `http://localhost:9200` | The target OpenSearch instance the proxy forwards requests to.             |
| `OPENSEARCH_URL`                 | `http://localhost:9200` | Base URL of the OpenSearch server (used internally by the proxy).          |
| `REVERSE_PROXY_BANNED_QUERY_PARAMS` | `q,query`             | Comma-separated list of query parameters that should be blocked.|
| `REVERSE_PROXY_PREFIX`           | `/proxy`                | URL prefix for routing proxied requests.                                   |
| `RUST_LOG`                       | `info`                  | Log level for the proxy (`error`, `warn`, `info`, `debug`, `trace`).       |


## Benchmark

You can use `hey` to benchmark the proxy server. First, [install](https://github.com/rakyll/hey) `hey` if you haven't already:

Running the benchmark:

```bash
hey -m POST \
  -H "Content-Type: application/json" \
  -d '{"query": {"match_all": {}}}' \
  http://localhost:3000/movies/_search
```

Results:

```bash
Summary:
  Total:	0.0651 secs
  Slowest:	0.0421 secs
  Fastest:	0.0013 secs
  Average:	0.0147 secs
  Requests/sec:	3071.3120
  
  Total data:	55800 bytes
  Size/request:	279 bytes

Response time histogram:
  0.001 [1]	  |■
  0.005 [11]	|■■■■■■■■
  0.009 [40]	|■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.014 [52]	|■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.018 [32]	|■■■■■■■■■■■■■■■■■■■■■■■■■
  0.022 [32]	|■■■■■■■■■■■■■■■■■■■■■■■■■
  0.026 [5]	  |■■■■
  0.030 [22]	|■■■■■■■■■■■■■■■■■
  0.034 [3]	  |■■
  0.038 [0]	  |
  0.042 [2]	  |■■


Latency distribution:
  10% in 0.0067 secs
  25% in 0.0091 secs
  50% in 0.0128 secs
  75% in 0.0191 secs
  90% in 0.0268 secs
  95% in 0.0288 secs
  99% in 0.0394 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.0010 secs, 0.0013 secs, 0.0421 secs
  DNS-lookup:	0.0003 secs, 0.0000 secs, 0.0020 secs
  req write:	0.0000 secs, 0.0000 secs, 0.0005 secs
  resp wait:	0.0137 secs, 0.0012 secs, 0.0372 secs
  resp read:	0.0000 secs, 0.0000 secs, 0.0001 secs

Status code distribution:
  [200]	200 responses
```
