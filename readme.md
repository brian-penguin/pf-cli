# Pathfinder CLI

Just a toy to practice Rust and Elasticsearch
- Parse a CSV
- deserialize into a Spell struct
- load Elasticsearch with spells
- allow for full text search and filtering
- cli it up!

### With Docker installed
Start up Elasticsearch in local mode with
```
 docker run -p 9200:9200 -p 9300:9300 -e "discovery.type=single-node" docker.elastic.co/elasticsearch/elasticsearch-oss:6.2.3
```

