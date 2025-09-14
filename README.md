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
