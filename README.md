# Simple Demo of Logging in Compute@Edge

This demo is intended to provide a quick example of a C@E service that logs JSON to a 3rd party endpoint.

Note: 
## Understanding the Code

This demo is intentional lightweight and is  demo intended to get you up and running quickly with logging. Refer to the Fastly docs for more information about configuring the logs.

The demo doesn't require the use of any backends. Once deployed, you will have a Fastly service running on Compute@Edge that can generate synthetic responses at the edge.

## Running the Service

See the Fastly docs for getting set up on Compute@Edge at edge. 

Once you have created a new Compute service and obtained a Fastly API key, copy the service name and ID (e.g. from the UI) into the `fastly.toml` file

```toml
name = "<your service name>"
service_id = "<your service ID>"
```

Configure an endpoint named `my_endpoint`. See the Fastly documentation for how to configuring an endpoint. Save the service with the configured endpoint

Deploy your service

``` sh
fastly compute publish
```

Start the log tail

```
fastly log-tail
```
