use elasticsearch::Elasticsearch;
use elasticsearch::{http::transport::Transport, IndexParts};

use elasticsearch::Error;

use serde_json::Value;

use elasticsearch::SearchParts;

#[macro_use]
extern crate serde_json;

static POSTS_INDEX: &'static str = "products";

async fn create_product_index(client: &Elasticsearch) -> Result<(), Error> {
    println!("Creating Elastic client ");
    let response = client
        .index(IndexParts::Index(POSTS_INDEX))
        // .create()
        .body(json!(
            {
              "mappings": {
                "properties": {
                  "type": {
                    "type": "keyword"
                  },
                  "id": {
                    "type": "integer"
                  },
                }
            },
            "settings": {
                "index.number_of_shards": 3,
            }
        }))
        .send()
        .await?;

    Ok(())
}

async fn insert_document(client: &Elasticsearch) -> Result<(), Error> {
    let response = client
        .index(IndexParts::Index(POSTS_INDEX))
        .body(json!({
            "type": "type of document",
            "id": "type of id ",

        }))
        .send()
        .await?;

    let successful = response.status_code().is_success();

    println!("{} succ", successful);
    Ok(())
}

async fn get_products_documents(client: &Elasticsearch) -> Result<(), Error> {
  let response = client
    .search(SearchParts::Index(&["products"]))
    .from(0)
    .size(10)
    .body(json!({
        "query": {
            "match_all": {
                
            }
        }
    }))
    .send()
    .await?;

let response_body = response.json::<Value>().await?;
let took = response_body["took"].as_i64().unwrap();
for hit in response_body["hits"]["hits"].as_array().unwrap() {
    // print the source document
    println!("{:?}", hit["_source"]);

  

}
Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = Transport::single_node("http://localhost:9200")?;
    let client = Elasticsearch::new(transport);

    // insert_document(&client).await;
    get_products_documents(&client).await;

    //    let _ =  create_product_index(&client).await;

    Ok(())
}
