# elastic-connector
This Fluvio connector is designed to seamlessly send streaming events to Elasticsearch, allowing for efficient real-time data indexing and search capabilities. By integrating Fluvio's event-driven architecture with Elasticsearch's powerful search engine, the connector enables the continuous flow of data from various sources into Elasticsearch, ensuring that events are captured, stored, and made searchable almost instantly, providing both scalability and flexibility in managing large datasets.
## Example Config
### config.yml
```
apiVersion: 0.1.0
meta:
  version: 0.1.0
  name: my-es-sink-connector-test-connector
  type: es-sink-connector-sink
  topic: testing
  secrets:
    - name: PASSWORD
    - name: USERNAME      
custom:
  url: http://127.0.0.1:9200
  index: test-index
  username: "${{ secrets.USERNAME }}"
  password: "${{ secrets.PASSWORD }}"
```
### secrets.txt
```
USERNAME=<THE_ELASTICSEARCH_USERNAME>
PASSWORD=<THE_ELASTICSEARCH_PASSWORD>
CLOUD_ID=<IF_YOU_ARE_USING_ELASTIC_CLOUD_THE_CLOUD_ID_OR_EMPTY>
```
## Running
```
cdk deploy --target <THE_RIGHT_TOOLCHAIN> start -c <THE_YAML_CONFIG_FILE> --secrets <THE_SECRETS_FILE>
```
example:
```
cdk deploy --target x86_64-unknown-linux-gnu start -c sample-config.yaml --secrets secrets.txt
```
## Produce Messages
With the Fluvio cluster up and running, you can begin producing messages by executing: 
```
fluvio produce testing
```
* Keep in mind that testing is the topic name and must match the topic specified in the configuration file.
## Message Formats
The message must be a valid JSON Object, for example:
```
{
    "name":"Test Name",
    "age": 42,
    "active": true
}
```