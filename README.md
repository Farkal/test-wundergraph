# Init
Just run `diesel database setup --database-url postgres://postgres:postgres@localhost/testwunder`

# Wundergraph generation
`wundergraph_cli print-schema postgres://postgres:postgres@localhost/testmovies > ./src/generated.rs`  
You just need to change Jsonb to serde_json::Value linked issue it here -> https://github.com/weiznich/wundergraph/issues/30

# Database scheme
![Database scheme](./test_data_model_v1.png)