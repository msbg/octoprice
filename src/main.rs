use serde::{Deserialize};
use serde_json::Result;
use reqwest::{Client, StatusCode, Url};

fn main() {
    println!("Hello, world!");
}


#[derive(Deserialize, Debug, PartialEq)]
pub struct Products {
    pub results: Vec<Product>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Product {
    code: String,
    display_name: String,
    brand: String 
}


fn get_product_from_string(all_products: String) -> Product {
    let p: Products = serde_json::from_str(all_products.as_str()).expect("Could not parse product json");
    let our_product: Vec<Product> = p.results.iter().filter(|x| {
        x.brand == "OCTOPUS_ENERGY" &&
        x.display_name == "Agile Octopus"
    }).map(|x| x.to_owned()).collect();
    assert_eq!(our_product.len(), 1);
    our_product[0].clone()
}

pub async fn get_products() -> String {
    let client = Client::new();
    let url = "https://api.octopus.energy//v1/products/";
    let response = client
        .get(url)
        .send()
        .await.expect("Response from octopus API");

    response.text().await.expect("Response body from Octopus API")
    
}

async fn get_product() ->Product {
    let product_string = get_products().await;
    print!("{}",product_string);
    get_product_from_string(product_string)
}

//struct ApiError {
//
//}

//fn get_products() -> Result<Products, ApiError> {
//
//} 

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, fs, clone};

    use super::*;

    #[test]
    fn deserialize_product() -> Result<()> {
        let data = r#"{
            "code": "AGILE-FLEX-22-11-25",
            "direction": "IMPORT",
            "full_name": "Agile Octopus November 2022 v1",
            "display_name": "Agile Octopus",
            "description": "With Agile Octopus, you get access to half-hourly energy prices, tied to wholesale prices and updated daily.  The unit rate is capped at 100p/kWh (including VAT).",
            "is_variable": true,
            "is_green": true,
            "is_tracker": false,
            "is_prepay": false,
            "is_business": false,
            "is_restricted": false,
            "term": null,
            "available_from": "2022-11-25T00:00:00Z",
            "available_to": null,
            "links": [
                {
                    "href": "https://api.octopus.energy/v1/products/AGILE-FLEX-22-11-25/",
                    "method": "GET",
                    "rel": "self"
                }
            ],
            "brand": "OCTOPUS_ENERGY"
        }"#;
        let p: Product = serde_json::from_str(data)?;
        assert_eq!(p,
            Product{
                code: "AGILE-FLEX-22-11-25".to_string(),
                display_name:"Agile Octopus".to_string(),
                brand:"OCTOPUS_ENERGY".to_string()
            }
        );
        Ok(())
    }

    #[test]
    fn deserialize_products() -> Result<()> {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources");
        d.push("test");
        d.push("sample_products.json");
        let full_path_to_test_data =  d.display().to_string();
        let data = fs::read_to_string(full_path_to_test_data).expect("Should have loaded file");
        let p: Products = serde_json::from_str(data.as_str())?;
        let mut product_example = Products {
            results: Vec::new(),
        };
        product_example.results.push(Product{
            code: "AGILE-FLEX-22-11-25".to_string(),
            display_name:"Agile Octopus".to_string(),
            brand:"OCTOPUS_ENERGY".to_string()
        });
        assert!(p.results.iter().any(|x| x.clone() == product_example.results[0]));
        let our_product: Vec<Product> = p.results.iter().filter(|x| {
            x.brand == "OCTOPUS_ENERGY" &&
            x.display_name == "Agile Octopus"
        }).map(|x| x.to_owned()).collect();
        assert_eq!(our_product.len(), 1);
        Ok(())
    }

    #[test]
    fn test_get_product_from_string() -> Result<()> {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources");
        d.push("test");
        d.push("sample_products.json");
        let full_path_to_test_data =  d.display().to_string();
        let data = fs::read_to_string(full_path_to_test_data).expect("Should have loaded file");
        let product = get_product_from_string(data);
        assert_eq!(product,
            Product{
                code: "AGILE-FLEX-22-11-25".to_string(),
                display_name:"Agile Octopus".to_string(),
                brand:"OCTOPUS_ENERGY".to_string()
            }
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_product() -> Result<()> {
        let product = get_product().await;
        assert_eq!(product,
            Product{
                code: "AGILE-FLEX-22-11-25".to_string(),
                display_name:"Agile Octopus".to_string(),
                brand:"OCTOPUS_ENERGY".to_string()
            }
        );
        Ok(())
    }

}
