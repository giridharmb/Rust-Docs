#### Notes

Rust Playground

https://play.rust-lang.org/

#### Openstack API : Fetch Compute Service List

This piece of code

- Makes HTTP Request To Keystone Endpoint To Get Token (Through Headers)
- Then make HTTP (Blocking) Request (GET) Call To Nova API Endpoint
- Fetches The Results & Stores As A JSON File

`Cargo.toml`

```toml
[package]
name = "openstack_worker_pool"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
reqwest = { version = "0.11.13" , features = ["blocking"] } # reqwest with JSON parsing support
tokio = { version = "1.24.1", features = ["full"] } # for our async runtime
futures = "0.3.4" # for our async / await blocks
serde = "1.0.152"
serde_json = "1.0.91"
serde_derive = "1.0.152"
rand = "0.8.5"
rayon = "1.6.1"
sha256 = "1.1.1"
```

How To Run

```bash
OUSER="<USER>" OPASS="<PASS>" cargo run
```

`src/main.rs`

```rust
use std::borrow::Borrow;
use std::collections::HashMap;

use serde::Deserialize;
use serde_json::{json, Value};
use std::env;
use std::fmt::{Display, Formatter};
use std::future::Future;
use futures::future::err;
use reqwest::{Error};

use reqwest::{Response, StatusCode};
use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderValue, ToStrError};
use reqwest::blocking::Client;

use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::time::Duration;
use rand::{distributions::Alphanumeric};
use sha256::digest;
use std::time::{SystemTime, UNIX_EPOCH};

use std::fs::File;

use std::time::Instant;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Cloud {
    region_name: String,
    region_endpoint: String
}

impl Cloud {

    fn get_region_endpoint(&self) -> String {
        let (_, my_map) = get_cloud_mapping_v2();

        match my_map.get(&self.region_name) {
            None => { "missing".to_owned() },
            Some(d) => {
                match my_map.get(&self.region_name) {
                    None => { "missing".to_owned() }
                    Some(d) => { d.to_string() },
                }
            }
        }
    }

    fn get_keystone_token(&self) -> String {
        return match self.exec_keystone_request() {
            Some(d) => {
                d
            }
            None => {
                println!(">> no token present");
                "".to_owned()
            }
        };
    }

    fn get_creds_payload(&self, user: &str, pass: &str) -> serde_json::Value {
        let payload_body = json!(
            {
                "auth":
                {
                    "identity":
                    {
                        "methods":
                        [
                            "password"
                        ],
                        "password":
                        {
                            "user":
                            {
                                "name": user,
                                "domain":
                                {
                                    "name": "Default"
                                },
                                "password": pass
                            }
                        }
                    }
                }
            }
        );
        return payload_body;
    }

    fn exec_keystone_request(&self) -> Option<String> {

        let ouser = get_openstack_user();
        if ouser == "" {
            return None
        }

        let opass = get_openstack_pass();
        if opass == "" {
            return None
        }

        let payload_body = self.get_creds_payload(ouser.as_str(), opass.as_str());

        let cloud_endpoint = self.get_region_endpoint();

        let keystone_url = format!("https://{}:5000/v3/auth/tokens", cloud_endpoint);

        let client = reqwest::blocking::Client::new();

        let payload_body_str = payload_body.to_string();

        let resp = client.post(keystone_url)
            .body(payload_body_str)
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .send();

        return match resp {
            Ok(d) => {
                Some(d.headers().get("X-Subject-Token").unwrap().to_str().unwrap().to_string())
            }
            Err(e) => {
                println!("error : {:?}", e);
                None
            }
        };
    }

    fn exec_compute_service_list_request(&self) -> Option<Value> {

        let token = self.get_keystone_token();
        if token == "" {
            println!("token missing, skipping this request...");
            return None
        }

        let cloud_endpoint = self.get_region_endpoint();

        let compute_service_request_url = format!("https://{}:8774/v2.1/os-services", cloud_endpoint);

        let client = reqwest::blocking::Client::new();

        let resp = client.get(compute_service_request_url)
            .header("X-Auth-Token", token)
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .send();

        return match resp {
            Ok(d) => {
                println!("data : {:#?}", d);
                let result_str = d.text().unwrap();
                let v: Value = serde_json::from_str(result_str.as_str()).unwrap();
                Some(v)
            }
            Err(e) => {
                println!("error : {:?}", e);
                None
            }
        };
    }
}

fn compute_job(cloud: Cloud) -> String {
    return match cloud.exec_keystone_request() {
        Some(d) => {
            d
        }
        None => {
            println!(">> no token present <<");
            "".to_owned()
        }
    };
}

fn compute_job_compute_service_list(cloud: Cloud) -> Value {
    return match cloud.exec_compute_service_list_request() {
        Some(d) => {
            d
        }
        None => {
            println!(">> no token present <<");
            let data = r#"{}"#;
            // Parse the string of data into serde_json::Value.
            let v: Value = serde_json::from_str(data).unwrap();
            v
        }
    };
}


fn generate_jobs() -> Vec<Cloud> {

    let mut jobs = vec![];

    let (cloud_region_mappings, _) = get_cloud_mapping_v2();

    for item in cloud_region_mappings {
        jobs.push(item);
    }
    return jobs;
}


fn get_cloud_mapping_v2() -> (Vec<Cloud>, HashMap<String, String>) {
    let vec_data = get_cloud_vector();
    let my_map = get_hashmap_v2(&vec_data);
    return (vec_data, my_map)
}

fn get_cloud_vector() -> Vec<Cloud> {

    let mut cloud_region_mappings = vec![];

    // replace this with your region endpoints

    cloud_region_mappings.push(Cloud{ region_name: "region01".parse().unwrap(), region_endpoint: "region01.endpoint.company.com".parse().unwrap() });
    cloud_region_mappings.push(Cloud{ region_name: "region02".parse().unwrap(), region_endpoint: "region02.endpoint.company.com".parse().unwrap() });
    cloud_region_mappings.push(Cloud{ region_name: "region03".parse().unwrap(), region_endpoint: "region03.endpoint.company.com".parse().unwrap() });
    cloud_region_mappings.push(Cloud{ region_name: "region04".parse().unwrap(), region_endpoint: "region04.endpoint.company.com".parse().unwrap() });
    cloud_region_mappings.push(Cloud{ region_name: "region05".parse().unwrap(), region_endpoint: "region05.endpoint.company.com".parse().unwrap() });

    return cloud_region_mappings
}

fn get_hashmap_v2(vec_data: &Vec<Cloud>) -> HashMap<String, String> {
    let mut my_map: HashMap<String, String> = HashMap::new();

    for data in vec_data {
        let (region_name, endpoint) = get_regionname_and_andpoint(data);
        my_map.insert(region_name, endpoint);
    }
    return my_map
}

fn get_regionname_and_andpoint(mapping: &Cloud) -> (String, String) {
    return (mapping.region_name.to_string(), mapping.region_endpoint.to_string())
}


fn get_openstack_user() -> String {
    let uname = "OUSER";
    return match env::var(uname) {
        Ok(v) => return v,
        Err(e) => {
            println!("env variable 'OUSER' is not set !");
            "".to_owned()
        }
    }
}

fn get_openstack_pass() -> String {
    let opass = "OPASS";
    return match env::var(opass) {
        Ok(v) => v,
        Err(e) => {
            println!("env variable 'OPASS' is not set !");
            "".to_owned()
        },
    };
}

fn main() {
    println!("OpenStack Test");

    rayon::ThreadPoolBuilder::new().num_threads(30).build_global().unwrap();

    let mut my_jobs = generate_jobs();

    let now = Instant::now();

    let results: Vec<Value> = my_jobs.into_par_iter()
        .map(compute_job_compute_service_list)
        .collect();

    let elapsed = now.elapsed();

    println!("results : {:#?}", results);

    println!("Elapsed: {:.2?}", elapsed);

    let f = match File::create("results.json") {
        Ok(d) => {
            let write_data = match serde_json::to_writer_pretty(&d, &results) {
                Ok(d) => {
                    println!("data successfully written to json file !");
                },
                Err(e) => {
                    println!("data could not be written to json file : {:?}", e);
                },
            };
        },
        Err(e) => {
            println!("json file could not be created : {:?}", e);
        },
    };
}
```

#### Azure : Get API & Graph API Token

How To Make API Calls To Public Azure Endpoints (Still, Work In Progress)

`Cargo.toml`

```toml
[package]
name = "azure-rust"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.5"
regex = "1.7.0"
reqwest = { version = "0.11.13" , features = ["blocking"] } # reqwest with JSON parsing support
tokio = { version = "1.23.0", features = ["full"] }
serde = "1.0.152"
serde_json = "1.0.91"
serde_derive = "1.0.152"
rayon = "1.6.1"
futures = "0.3.4" # for our async / await blocks
```

File `src/main.rs`

```rust
use std::collections::HashMap;
use reqwest::{Response, StatusCode};
use serde_json::{Map, Value};


#[derive(Debug)]
pub enum GenericError {
    InvalidServicePrincipal,
    MissingDataInResponse,
}

#[derive(Debug)]
struct CustomError {
    err_type: GenericError,
    err_msg: String
}

enum TokenType {
    AzureApiToken,
    AzureGraphToken,
}

// pub type ResultToken = Result<String, CustomError>;

struct ServicePrincipal {
    client_id: String,
    client_secret: String,
    tenant_id: String,
    region: String,
    token_type: TokenType
}

impl ServicePrincipal {
    fn get_token(&self, token_type: TokenType) -> Result<String, CustomError> {
        let region = &self.region;
        let client_id = &self.client_id;
        let client_secret = &self.client_secret;
        let region = &self.region;
        let tenant_id = &self.tenant_id;

        let azure_region_details = get_azure_region_details(region);

        let authority_url = match token_type {
            TokenType::AzureApiToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/token";
                authority_url
            },
            TokenType::AzureGraphToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/v2.0/token";
                authority_url
            },
        };

        println!("authority_url : {:?}", authority_url);

        let resource_api: String = azure_region_details.get("resourceAPI").unwrap().to_string();

        let scope = azure_region_details.get("resourceGraphURL").unwrap().to_string();

        println!("resource_api : {:?}", resource_api);

        let params = match token_type {
            TokenType::AzureApiToken => {
                let params = [
                    ("grant_type", "client_credentials"),
                    ("client_id", client_id),
                    ("client_secret", client_secret),
                    ("resource", ""),
                ];
                params
            },
            TokenType::AzureGraphToken => {
                let params = [
                    ("grant_type", "client_credentials"),
                    ("client_id", client_id),
                    ("client_secret", client_secret),
                    ("scope", scope.as_str()),
                ];
                params
            }
        };


        let client = reqwest::blocking::Client::new();

        let res = client.post(authority_url).form(&params).send();

        let result =  match res {
            Ok(res) => {
                println!("http response : {:?}", res);
                let data_str = res.text().unwrap_or("N/A".to_string());
                let value: Value = serde_json::from_str(data_str.as_str()).unwrap();

                let my_object = match value.as_object() {
                    None => {
                        println!("no object found")
                    },
                    Some(d) => {
                        if d.contains_key("token_type") == false || d.contains_key("access_token") == false {
                            let msg:String = format!("http response does not contains either the follow keys : (token_type|access_token)");
                            let custom_err = CustomError {
                                err_msg: String::from(msg),
                                err_type: GenericError::MissingDataInResponse
                            };
                            return Err(custom_err)
                        }
                    }
                };

                let token = format!("{} {}", &value["token_type"].as_str().unwrap(), &value["access_token"].as_str().unwrap());
                token
            },
            Err(err) => {
                let msg:String = format!("there was an error http form submit to get token {:?}", err);
                let custom_err = CustomError {
                    err_msg: String::from(msg),
                    err_type: GenericError::InvalidServicePrincipal
                };
                return Err(custom_err)
            }
        };
        Ok(result)
    }
}


fn get_azure_region_details(region: &str) -> HashMap<String, String> {
    let mut azure_region: HashMap<String, String> = HashMap::new();

    return match region {
        "america" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        },
        "china" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.chinacloudapi.cn".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.partner.microsoftonline.cn".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://microsoftgraph.chinacloudapi.cn/.default".to_owned());
            azure_region
        },
        _ => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        }
    }
}

fn main() {
    println!("Azure Data >");

    let service_principal = ServicePrincipal {
        client_id: "00000000-0000-0000-0000-000000000000".to_owned(),
        client_secret: "<INSERT_SECRET_HERE>".to_owned(),
        region: "america".to_owned(),
        tenant_id: "00000000-0000-0000-0000-000000000000".to_owned(),
        token_type: TokenType::AzureApiToken,
    };

    // api token
    let api_token = match service_principal.get_token(TokenType::AzureApiToken) {
        Ok(data) => {
            println!("\ntoken\n");
            println!("{}\n", data);
        }
        Err(e) => {
            println!("error : {:#?}", e);
        }
    };

    // graph token
    let graph_token = match service_principal.get_token(TokenType::AzureGraphToken) {
        Ok(data) => {
            println!("\ntoken\n");
            println!("{}\n", data);
        }
        Err(e) => {
            println!("error : {:#?}", e);
        }
    };
}
```

#### Public Azure Sample

Fetching Virtual Machines : Azure Resource Graph Query (ADX) : (FYI : Still, Work In Progress) : Part-1

```rust
use std::collections::HashMap;
use reqwest::{Response, StatusCode};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Map, Value};
use std::{thread, time};

#[derive(Debug)]
pub enum GenericError {
    InvalidServicePrincipal,
    MissingDataInResponse,
}

#[derive(Debug)]
struct CustomError {
    err_type: GenericError,
    err_msg: String
}

enum TokenType {
    AzureApiToken,
    AzureGraphToken,
}

// pub type ResultToken = Result<String, CustomError>;

struct ServicePrincipal {
    client_id: String,
    client_secret: String,
    tenant_id: String,
    region: String,
    token_type: TokenType
}

// microsoft public api endpoints

fn get_azure_region_details(region: &str) -> HashMap<String, String> {
    let mut azure_region: HashMap<String, String> = HashMap::new();

    return match region {
        "america" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        },
        "china" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.chinacloudapi.cn".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.partner.microsoftonline.cn".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://microsoftgraph.chinacloudapi.cn/.default".to_owned());
            azure_region
        },
        _ => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        }
    }
}


impl ServicePrincipal {

    fn get_params_and_url(&self, token_type: TokenType, azure_scope: Option<String>) -> ([(String, String); 4], String, String, String) {
        let region = &self.region;
        let client_id = &self.client_id;
        let tenant_id = &self.tenant_id;
        let client_secret = &self.client_secret;

        let azure_region_details = get_azure_region_details(region.as_str());

        let authority_url = match token_type {
            TokenType::AzureApiToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/token";
                authority_url
            },
            TokenType::AzureGraphToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/v2.0/token";
                authority_url
            },
        };

        let resource_graph_url = azure_region_details.get("resourceGraphURL").unwrap().to_string();

        let resource_api_url = azure_region_details.get("resourceAPI").unwrap().to_string();

        // println!("authority_url      : {:?}", authority_url);
        // println!("resource_graph_url : {:?}", resource_graph_url);
        // println!("resource_api_url   : {:?}", resource_api_url);

        let my_scope = match azure_scope {
            None => {
                println!("azure_scope is None !");
                "https://graph.microsoft.com/.default".to_owned()
            },
            Some(d) => {
                d.to_owned()
            },
        };

        let params = match token_type {
            TokenType::AzureApiToken => {
                let params = [
                    ("grant_type".to_string(), "client_credentials".to_string()),
                    ("client_id".to_string(), client_id.to_string()),
                    ("client_secret".to_string(), client_secret.to_string()),
                    ("resource".to_string(), "".to_string()),
                ];
                params
            },
            TokenType::AzureGraphToken => {
                let params = [
                    ("grant_type".to_string(), "client_credentials".to_string()),
                    ("client_id".to_string(), client_id.to_string()),
                    ("client_secret".to_string(), client_secret.to_string()),
                    ("scope".to_string(), my_scope.to_string()),
                ];
                params
            }
        };
        let return_data = (params, authority_url, resource_graph_url, resource_api_url);
        // println!("{:#?}", &return_data);
        return return_data;
    }

    fn get_token(&self, token_type: TokenType, azure_scope: Option<String>) -> Result<String, CustomError> {
        let region = &self.region;
        let client_id = &self.client_id;
        let client_secret = &self.client_secret;
        let region = &self.region;
        let tenant_id = &self.tenant_id;

        let my_scope = Some("https://management.azure.com/.default".to_string());

        let my_scope = match azure_scope {
            None => {
                println!("azure_scope is None !");
                "https://graph.microsoft.com/.default".to_owned()
            },
            Some(d) => {
                d.to_owned()
            },
        };

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(token_type, Some(my_scope));

        let client = reqwest::blocking::Client::new();

        let res = client.post(authority_url).form(&params).send();

        let result =  match res {
            Ok(res) => {
                println!("http response : {:?}", res);
                let data_str = res.text().unwrap_or("N/A".to_string());
                let value: Value = serde_json::from_str(data_str.as_str()).unwrap();

                let my_object = match value.as_object() {
                    None => {
                        println!("no object found");
                    },
                    Some(d) => {
                        if d.contains_key("token_type") == false || d.contains_key("access_token") == false {
                            let msg:String = format!("http response does not contains either the follow keys : (token_type|access_token)");
                            let custom_err = CustomError {
                                err_msg: String::from(msg),
                                err_type: GenericError::MissingDataInResponse
                            };
                            return Err(custom_err)
                        }
                    }
                };

                let token = format!("{} {}", &value["token_type"].as_str().unwrap(), &value["access_token"].as_str().unwrap());
                token
            },
            Err(err) => {
                let msg:String = format!("there was an error http form submit to get token {:?}", err);
                let custom_err = CustomError {
                    err_msg: String::from(msg),
                    err_type: GenericError::InvalidServicePrincipal
                };
                return Err(custom_err)
            }
        };
        Ok(result)
    }

    fn get_vms(&self) {

        let my_scope = Some("https://management.azure.com/.default".to_owned());

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(TokenType::AzureGraphToken, my_scope.to_owned());

        let api_url = resource_api_url.to_owned() + "/providers/Microsoft.ResourceGraph/resources?api-version=2020-04-01-preview";

        println!("api_url for azure_data_explorer : {}", api_url.as_str());

        let empty:Vec<String> = vec![];

        /*
            0 - 999      -> first page
            1000 - 1999  -> second page
            2000 - 2999  -> third page
            3000 - 3999  -> fourth page

            $top -> The maximum number of rows that the query should return. Overrides the page size when $skipToken property is present.
            $skip -> The number of rows to skip from the beginning of the results. Overrides the next page offset when $skipToken property is present.
        */

        let json_payload = json!({
            "subscriptions": empty,
            "options" : {
                "$top" : 1000, // usually fixed at 1000
                "$skip" : 0
            },
            "query": "Resources | where type =~ 'Microsoft.Compute/virtualMachines'"

            // "query": "Resources | project id, name, type, location | where type =~ 'Microsoft.Compute/virtualMachines' | limit 10"
        });

        println!("json_payload:");
        println!("{:#?}", json_payload);

        let token_data = match self.get_token(TokenType::AzureGraphToken, my_scope.to_owned()) {
            Ok(d) => {
                let client = reqwest::blocking::Client::new();

                let token = d.to_owned();
                println!("token for azure_data_explorer : {}", token.as_str());

                let now = time::Instant::now();

                let res = match client.post(api_url.as_str()).body(json_payload.to_string()).header(CONTENT_TYPE, "application/json").header(AUTHORIZATION, token.as_str()).send() {
                    Ok(d) => {
                        let elapsed = now.elapsed();

                        println!("azure_data_explorer query result:");
                        let json_str = d.text().unwrap();
                        let json_data: Value = serde_json::from_str(json_str.as_str()).unwrap();

                        // uncomment this to see the records
                        // println!("{:#?}", json_data["data"]["rows"]);

                        let total_records = &json_data["data"]["rows"].as_array().unwrap().len();
                        println!("Total Records Fetched : {}",  total_records);
                        println!("Total Time Taken : {:.2?}", elapsed);
                    },
                    Err(e) => {
                        println!("error : could not make http request for adx query : {:?}", e);
                    },
                };
            },
            Err(e) => {
                println!("error : could not fetch token : {:#?}", e);
            },
        };
    }
}

fn main() {
    println!("Azure Data >");

    let service_principal = ServicePrincipal {
        client_id: "00000000-0000-0000-0000-000000000000".to_owned(),
        client_secret: "<INSERT_SECRET_HERE>".to_owned(),
        region: "america".to_owned(),
        tenant_id: "00000000-0000-0000-0000-000000000000".to_owned(),
        token_type: TokenType::AzureApiToken,
    };

    // api token
    let api_token = match service_principal.get_token(TokenType::AzureApiToken, None) {
        Ok(data) => {
            println!("\ntoken\n");
            println!("{}\n", data);
        }
        Err(e) => {
            println!("error : {:#?}", e);
        }
    };

    // graph token
    let graph_token = match service_principal.get_token(TokenType::AzureGraphToken, None) {
        Ok(data) => {
            println!("\ntoken\n");
            println!("{}\n", data);
        }
        Err(e) => {
            println!("error : {:#?}", e);
        }
    };

    service_principal.get_vms();
}
```

#### Split Into Multiple Chunks

```rust
fn main() {
    let input: Vec<_> = (0..27).collect();
    let result: Vec<_> = input.chunks(5).collect();
    println!("{:?}", result);
}
```

Output

```bash
[
    [
        0,
        1,
        2,
        3,
        4,
    ],
    [
        5,
        6,
        7,
        8,
        9,
    ],
    [
        10,
        11,
        12,
        13,
        14,
    ],
    [
        15,
        16,
        17,
        18,
        19,
    ],
    [
        20,
        21,
        22,
        23,
        24,
    ],
    [
        25,
        26,
    ],
]
```

#### Public Azure Sample

Fetching Virtual Machines : Azure Resource Graph Query (ADX) : (FYI : Still, Work In Progress) : Part-2

How To Run

```bash
cargo build --release
```

```bash
CLIENT_ID="00000000-0000-0000-0000-000000000000" CLIENT_SECRET="<INSERT_SECRET_HERE>" TENANT_ID="00000000-0000-0000-0000-000000000000" REGION="america" target/release/azure-rust
```

`src/main.rs`

```rust
use std::collections::HashMap;
use reqwest::{Response, StatusCode};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Map, Value};
use std::{env, thread, time};
use std::slice::Chunks;

#[derive(Debug)]
pub enum GenericError {
    InvalidServicePrincipal,
    MissingDataInResponse,
}

#[derive(Debug)]
struct CustomError {
    err_type: GenericError,
    err_msg: String
}

enum TokenType {
    AzureApiToken,
    AzureGraphToken,
}

#[derive(Clone, Copy)]
enum AzureRecord {
    VirtualMachine
}

// pub type ResultToken = Result<String, CustomError>;

struct ServicePrincipal {
    client_id: String,
    client_secret: String,
    tenant_id: String,
    region: String,
    token_type: TokenType
}

// microsoft public api endpoints

fn get_azure_region_details(region: &str) -> HashMap<String, String> {
    let mut azure_region: HashMap<String, String> = HashMap::new();

    return match region {
        "america" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        },
        "china" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.chinacloudapi.cn".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.partner.microsoftonline.cn".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://microsoftgraph.chinacloudapi.cn/.default".to_owned());
            azure_region
        },
        _ => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        }
    }
}

fn get_azure_record(azure_record: AzureRecord) -> String {
    return match azure_record {
        AzureRecord::VirtualMachine => "microsoft.compute/virtualmachines".to_owned(),
    }
}

impl ServicePrincipal {

    fn get_params_and_url(&self, token_type: TokenType, azure_scope: Option<String>) -> ([(String, String); 4], String, String, String) {
        let region = &self.region;
        let client_id = &self.client_id;
        let tenant_id = &self.tenant_id;
        let client_secret = &self.client_secret;

        let azure_region_details = get_azure_region_details(region.as_str());

        let authority_url = match token_type {
            TokenType::AzureApiToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/token";
                authority_url
            },
            TokenType::AzureGraphToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/v2.0/token";
                authority_url
            },
        };

        let resource_graph_url = azure_region_details.get("resourceGraphURL").unwrap().to_string();

        let resource_api_url = azure_region_details.get("resourceAPI").unwrap().to_string();

        // println!("authority_url      : {:?}", authority_url);
        // println!("resource_graph_url : {:?}", resource_graph_url);
        // println!("resource_api_url   : {:?}", resource_api_url);

        let my_scope = match azure_scope {
            None => {
                println!("azure_scope is None !");
                "https://graph.microsoft.com/.default".to_owned()
            },
            Some(d) => {
                d.to_owned()
            },
        };

        let params = match token_type {
            TokenType::AzureApiToken => {
                let params = [
                    ("grant_type".to_string(), "client_credentials".to_string()),
                    ("client_id".to_string(), client_id.to_string()),
                    ("client_secret".to_string(), client_secret.to_string()),
                    ("resource".to_string(), "".to_string()),
                ];
                params
            },
            TokenType::AzureGraphToken => {
                let params = [
                    ("grant_type".to_string(), "client_credentials".to_string()),
                    ("client_id".to_string(), client_id.to_string()),
                    ("client_secret".to_string(), client_secret.to_string()),
                    ("scope".to_string(), my_scope.to_string()),
                ];
                params
            }
        };
        let return_data = (params, authority_url, resource_graph_url, resource_api_url);
        // println!("{:#?}", &return_data);
        return return_data;
    }

    fn get_token(&self, token_type: TokenType, azure_scope: Option<String>) -> Result<String, CustomError> {
        let region = &self.region;
        let client_id = &self.client_id;
        let client_secret = &self.client_secret;
        let region = &self.region;
        let tenant_id = &self.tenant_id;

        let my_scope = Some("https://management.azure.com/.default".to_string());

        let my_scope = match azure_scope {
            None => {
                println!("azure_scope is None !");
                "https://graph.microsoft.com/.default".to_owned()
            },
            Some(d) => {
                d.to_owned()
            },
        };

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(token_type, Some(my_scope));

        let client = reqwest::blocking::Client::new();

        let res = client.post(authority_url).form(&params).send();

        let result =  match res {
            Ok(res) => {
                println!("http response : {:?}", res);
                let data_str = res.text().unwrap_or("N/A".to_string());
                let value: Value = serde_json::from_str(data_str.as_str()).unwrap();

                let my_object = match value.as_object() {
                    None => {
                        println!("no object found");
                    },
                    Some(d) => {
                        if d.contains_key("token_type") == false || d.contains_key("access_token") == false {
                            let msg:String = format!("http response does not contains either the follow keys : (token_type|access_token)");
                            let custom_err = CustomError {
                                err_msg: String::from(msg),
                                err_type: GenericError::MissingDataInResponse
                            };
                            return Err(custom_err)
                        }
                    }
                };

                let token = format!("{} {}", &value["token_type"].as_str().unwrap(), &value["access_token"].as_str().unwrap());
                token
            },
            Err(err) => {
                let msg:String = format!("there was an error http form submit to get token {:?}", err);
                let custom_err = CustomError {
                    err_msg: String::from(msg),
                    err_type: GenericError::InvalidServicePrincipal
                };
                return Err(custom_err)
            }
        };
        Ok(result)
    }

    fn get_total_records(&self, azure_record: AzureRecord) -> u64 {
        let my_scope = Some("https://management.azure.com/.default".to_owned());

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(TokenType::AzureGraphToken, my_scope.to_owned());

        let api_url = resource_api_url.to_owned() + "/providers/Microsoft.ResourceGraph/resources?api-version=2020-04-01-preview";

        println!("api_url for azure_data_explorer : {}", api_url.as_str());

        let empty:Vec<String> = vec![];

        let record_name = get_azure_record(azure_record);

        let total_records_query = format!("Resources | where type == '{}' | summarize count()", record_name);

        let json_payload = json!({
            "subscriptions": empty,
            "query": total_records_query
        });

        println!("json_payload:");
        println!("{:#?}", json_payload);

        let total_records = return match self.get_token(TokenType::AzureGraphToken, my_scope.to_owned()) {
            Ok(d) => {
                let client = reqwest::blocking::Client::new();
                let token = d.to_owned();
                println!("token for azure_data_explorer : {}", token.as_str());
                let now = time::Instant::now();
                let res = match client.post(api_url.as_str()).body(json_payload.to_string()).header(CONTENT_TYPE, "application/json").header(AUTHORIZATION, token.as_str()).send() {
                    Ok(d) => {
                        let elapsed = now.elapsed();
                        let json_str = d.text().unwrap();
                        let json_data: Value = serde_json::from_str(json_str.as_str()).unwrap();
                        println!("{}", serde_json::to_string_pretty(&json_data).unwrap());
                        let total_records = &json_data["data"]["rows"][0][0].as_u64().unwrap();
                        println!("total_records : {}", total_records);
                        println!("TTT : {:.2?}", elapsed);
                        total_records.to_owned()
                    },
                    Err(e) => {
                        println!("error : could not make http request for adx query : {:?}", e);
                        0 as u64
                    },
                };
                res
            }, // Ok
            Err(e) => {
                println!("error : could not fetch token : {:#?}", e);
                0
            }, // Err
        };
        total_records
    }

    fn get_azure_records(&self, azure_record: AzureRecord, top: i32, skip: i32) -> Vec<Value> {

        let my_scope = Some("https://management.azure.com/.default".to_owned());

        let empty_vec:Vec<Value> = Vec::new();

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(TokenType::AzureGraphToken, my_scope.to_owned());

        let api_url = resource_api_url.to_owned() + "/providers/Microsoft.ResourceGraph/resources?api-version=2020-04-01-preview";

        println!("api_url for azure_data_explorer : {}", api_url.as_str());

        let empty:Vec<String> = vec![];

        /*
            0 - 999      -> first page
            1000 - 1999  -> second page
            2000 - 2999  -> third page
            3000 - 3999  -> fourth page

            $top -> The maximum number of rows that the query should return. Overrides the page size when $skipToken property is present.
            $skip -> The number of rows to skip from the beginning of the results. Overrides the next page offset when $skipToken property is present.
        */

        let azure_entity = get_azure_record(azure_record);

        let query = format!("Resources | where type =~ '{}'", azure_entity);

        let json_payload = json!({
            "subscriptions": empty,
            "options" : {
                "$top" : top, // usually fixed at 1000
                "$skip" : skip
            },
            "query": query
        });

        println!("{}", serde_json::to_string_pretty(&json_payload).unwrap());

        let azure_data = match self.get_token(TokenType::AzureGraphToken, my_scope.to_owned()) {
            Ok(d) => {
                let client = reqwest::blocking::Client::new();
                let token = d.to_owned();
                let now = time::Instant::now();
                let res = match client.post(api_url.as_str()).body(json_payload.to_string()).header(CONTENT_TYPE, "application/json").header(AUTHORIZATION, token.as_str()).send() {
                    Ok(d) => {
                        let elapsed = now.elapsed();

                        println!("azure_data_explorer query result:");
                        let json_str = d.text().unwrap();
                        let json_data: Value = serde_json::from_str(json_str.as_str()).unwrap();
                        let rows = &json_data["data"]["rows"];


                        println!("TTT (top : {} , skip : {}) : {:.2?}", top, skip, elapsed);
                        rows.as_array().unwrap().to_vec()
                    },
                    Err(e) => {
                        println!("error : could not make http request for adx query : {:?}", e);
                        empty_vec
                    },
                };
                res.to_owned()
            },
            Err(e) => {
                println!("error : could not fetch token : {:#?}", e);
                empty_vec
            },
        };
        azure_data
    }

    fn get_all_azure_records(&self, azure_record: AzureRecord, records_per_page: i32, write_json_to_file: bool) -> Vec<Value> {
        let total_records_for_virtual_machines = self.get_total_records(azure_record);

        // let page_number = 1;

        // let records_per_page = 1000;

        let total_pages = (total_records_for_virtual_machines as f32 / records_per_page as f32).ceil() as i32;

        println!("records_per_page : {}", records_per_page);
        println!("total_pages : {}", total_pages);

        let mut all_records: Vec<Value> = Vec::new();

        let now = time::Instant::now();

        let mut page_list: Vec<i32> = Vec::new();

        for i in 1..=total_pages {
            page_list.push(i);
        }

        let result: Vec<_> = page_list.chunks(8).collect();
        println!("{:#?}", result);

        for page_number in 1..=total_pages {
            println!("fetching data for page_number : {}...", page_number);

            let skip = get_skip_number(records_per_page, page_number);

            let records = self.get_azure_records(AzureRecord::VirtualMachine, records_per_page, skip);
            println!("total records for page_number {} : {}", page_number, records.len());
            for item in records.iter() {
                &all_records.push(item.to_owned());
            }
            println!("iteration in progress : length of all_records : {}", &all_records.len());
            let elapsed = now.elapsed();
            println!("total time taken so far : {:.2?}", elapsed);
        }

        println!("length of all_records : {}", &all_records.len());

        if write_json_to_file {
            let azure_entity = get_azure_record(azure_record);
            let output_file = azure_entity.replace("/", "_").replace(".", "_") + ".json";

            std::fs::write(
            output_file,
        serde_json::to_string_pretty(&all_records).unwrap(),
            ).unwrap();
        }
        all_records
    }
}

enum EnvironmentVarible {
    ClientID,
    ClientSecret,
    TenantID,
    Region,
}

fn get_env_var(env_var: EnvironmentVarible) -> String {
    return match env_var {
        EnvironmentVarible::ClientID => {
            match env::var("CLIENT_ID") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'CLIENT_ID' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::ClientSecret => {
            match env::var("CLIENT_SECRET") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'CLIENT_SECRET' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::TenantID => {
            match env::var("TENANT_ID") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'TENANT_ID' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::Region => {
            match env::var("REGION") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'TENANT_ID' is not set !");
                    "".to_owned()
                },
            }
        }
    };
}

fn main() {
    println!("Azure Data >");

    let client_id = get_env_var(EnvironmentVarible::ClientID);
    let client_secret = get_env_var(EnvironmentVarible::ClientSecret);
    let tenant_id = get_env_var(EnvironmentVarible::TenantID);
    let region = get_env_var(EnvironmentVarible::Region);

    let service_principal = ServicePrincipal {
        client_id,
        client_secret,
        region,
        tenant_id,
        token_type: TokenType::AzureApiToken,
    };

    // api token
    let api_token = match service_principal.get_token(TokenType::AzureApiToken, None) {
        Ok(data) => {
            println!("\ntoken\n");
            println!("{}\n", data);
        }
        Err(e) => {
            println!("error : {:#?}", e);
        }
    };

    // graph token
    let graph_token = match service_principal.get_token(TokenType::AzureGraphToken, None) {
        Ok(data) => {
            println!("\ntoken\n");
            println!("{}\n", data);
        }
        Err(e) => {
            println!("error : {:#?}", e);
        }
    };

    let all_recs = service_principal.get_all_azure_records(AzureRecord::VirtualMachine,1000,true);
    println!("all_recs.len() : {}", all_recs.len())

}

fn get_skip_number(records_per_page: i32, page_number: i32) -> i32 {
    (page_number-1) * records_per_page
}
```

Fetching Virtual Machines : Azure Resource Graph Query (ADX) , Using Cache : (FYI : Still, Work In Progress) : Part-3

`src/main.rs`

```rust
use std::collections::HashMap;
use reqwest::{Response, StatusCode};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Map, Value};
use std::{env, thread, time};
use std::borrow::Borrow;
use std::slice::Chunks;

use moka::sync::Cache;
use std::time::Duration;

use lazy_static::lazy_static;


#[derive(Debug)]
pub enum GenericError {
    InvalidServicePrincipal,
    MissingDataInResponse,
}

#[derive(Debug)]
struct CustomError {
    err_type: GenericError,
    err_msg: String
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum TokenType {
    AzureApiToken,
    AzureGraphToken,
}

#[derive(Clone, Copy)]
enum AzureRecord {
    VirtualMachine
}

// pub type ResultToken = Result<String, CustomError>;

struct ServicePrincipal {
    client_id: String,
    client_secret: String,
    tenant_id: String,
    region: String,
    token_type: TokenType
}

// global variable

lazy_static! {
    static ref APP_CACHE:Cache<TokenAndScope, String> = Cache::builder()
        // Time to live (TTL): 45 minutes
        .time_to_live(Duration::from_secs(45 * 60))
        // Time to idle (TTI):  45 minutes
        .time_to_idle(Duration::from_secs(45 * 60))
        // Create the cache.
        .build();
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct TokenAndScope {
    token_type: TokenType,
    azure_scope: Option<String>
}

// microsoft public api endpoints

fn get_azure_region_details(region: &str) -> HashMap<String, String> {
    let mut azure_region: HashMap<String, String> = HashMap::new();

    return match region {
        "america" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        },
        "china" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.chinacloudapi.cn".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.partner.microsoftonline.cn".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://microsoftgraph.chinacloudapi.cn/.default".to_owned());
            azure_region
        },
        _ => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        }
    }
}

fn get_azure_record(azure_record: AzureRecord) -> String {
    return match azure_record {
        AzureRecord::VirtualMachine => "microsoft.compute/virtualmachines".to_owned(),
    }
}

impl ServicePrincipal {

    fn get_params_and_url(&self, token_type: TokenType, azure_scope: Option<String>) -> ([(String, String); 4], String, String, String) {
        let region = &self.region;
        let client_id = &self.client_id;
        let tenant_id = &self.tenant_id;
        let client_secret = &self.client_secret;

        let azure_region_details = get_azure_region_details(region.as_str());

        let authority_url = match token_type {
            TokenType::AzureApiToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/token";
                authority_url
            },
            TokenType::AzureGraphToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/v2.0/token";
                authority_url
            },
        };

        let resource_graph_url = azure_region_details.get("resourceGraphURL").unwrap().to_string();

        let resource_api_url = azure_region_details.get("resourceAPI").unwrap().to_string();

        // println!("authority_url      : {:?}", authority_url);
        // println!("resource_graph_url : {:?}", resource_graph_url);
        // println!("resource_api_url   : {:?}", resource_api_url);

        let my_scope = match azure_scope {
            None => {
                println!("azure_scope is None !");
                "https://graph.microsoft.com/.default".to_owned()
            },
            Some(d) => {
                d.to_owned()
            },
        };

        let params = match token_type {
            TokenType::AzureApiToken => {
                let params = [
                    ("grant_type".to_string(), "client_credentials".to_string()),
                    ("client_id".to_string(), client_id.to_string()),
                    ("client_secret".to_string(), client_secret.to_string()),
                    ("resource".to_string(), "".to_string()),
                ];
                params
            },
            TokenType::AzureGraphToken => {
                let params = [
                    ("grant_type".to_string(), "client_credentials".to_string()),
                    ("client_id".to_string(), client_id.to_string()),
                    ("client_secret".to_string(), client_secret.to_string()),
                    ("scope".to_string(), my_scope.to_string()),
                ];
                params
            }
        };
        let return_data = (params, authority_url, resource_graph_url, resource_api_url);
        // println!("{:#?}", &return_data);
        return return_data;
    }

    fn get_token(&self, token_type: TokenType, azure_scope: Option<String>) -> Result<String, CustomError> {

        let token_type_and_scope = TokenAndScope { token_type: token_type.to_owned(), azure_scope: azure_scope.to_owned() };

        let my_token = match APP_CACHE.get(&token_type_and_scope) {
            None => {
                println!("MISSING_TOKEN in APP_CACHE");
                "".to_owned()
            }
            Some(d) => {
                println!("APP_CACHE : token : {}", d);
                d
            }
        };

        if my_token != "" {
            return Ok(my_token)
        }

        let region = &self.region;
        let client_id = &self.client_id;
        let client_secret = &self.client_secret;
        let region = &self.region;
        let tenant_id = &self.tenant_id;

        let my_scope = Some("https://management.azure.com/.default".to_string());

        let my_scope = match azure_scope {
            None => {
                println!("azure_scope is None !");
                "https://graph.microsoft.com/.default".to_owned()
            },
            Some(d) => {
                d.to_owned()
            },
        };

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(token_type, Some(my_scope));

        let client = reqwest::blocking::Client::new();

        let res = client.post(authority_url).form(&params).send();

        let result =  match res {
            Ok(res) => {
                println!("http response : {:?}", res);
                let data_str = res.text().unwrap_or("N/A".to_string());
                let value: Value = serde_json::from_str(data_str.as_str()).unwrap();

                let my_object = match value.as_object() {
                    None => {
                        println!("no object found");
                    },
                    Some(d) => {
                        if d.contains_key("token_type") == false || d.contains_key("access_token") == false {
                            let msg:String = format!("http response does not contains either the follow keys : (token_type|access_token)");
                            let custom_err = CustomError {
                                err_msg: String::from(msg),
                                err_type: GenericError::MissingDataInResponse
                            };
                            return Err(custom_err)
                        }
                    }
                };

                let token = format!("{} {}", &value["token_type"].as_str().unwrap(), &value["access_token"].as_str().unwrap());
                token
            },
            Err(err) => {
                let msg:String = format!("there was an error http form submit to get token {:?}", err);
                let custom_err = CustomError {
                    err_msg: String::from(msg),
                    err_type: GenericError::InvalidServicePrincipal
                };
                return Err(custom_err)
            }
        };
        println!("Storing api_token : {} , in APP_CACHE", result.to_owned());
        APP_CACHE.insert(token_type_and_scope, result.to_owned());
        Ok(result)
    }

    fn get_total_records(&self, azure_record: AzureRecord) -> u64 {
        let my_scope = Some("https://management.azure.com/.default".to_owned());

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(TokenType::AzureGraphToken, my_scope.to_owned());

        let api_url = resource_api_url.to_owned() + "/providers/Microsoft.ResourceGraph/resources?api-version=2020-04-01-preview";

        println!("api_url for azure_data_explorer : {}", api_url.as_str());

        let empty:Vec<String> = vec![];

        let record_name = get_azure_record(azure_record);

        let total_records_query = format!("Resources | where type == '{}' | summarize count()", record_name);

        let json_payload = json!({
            "subscriptions": empty,
            "query": total_records_query
        });

        println!("json_payload:");
        println!("{:#?}", json_payload);

        let total_records = return match self.get_token(TokenType::AzureGraphToken, my_scope.to_owned()) {
            Ok(d) => {
                let client = reqwest::blocking::Client::new();
                let token = d.to_owned();
                println!("token for azure_data_explorer : {}", token.as_str());
                let now = time::Instant::now();
                let res = match client.post(api_url.as_str()).body(json_payload.to_string()).header(CONTENT_TYPE, "application/json").header(AUTHORIZATION, token.as_str()).send() {
                    Ok(d) => {
                        let elapsed = now.elapsed();
                        let json_str = d.text().unwrap();
                        let json_data: Value = serde_json::from_str(json_str.as_str()).unwrap();
                        println!("{}", serde_json::to_string_pretty(&json_data).unwrap());
                        let total_records = &json_data["data"]["rows"][0][0].as_u64().unwrap();
                        println!("total_records : {}", total_records);
                        println!("TTT : {:.2?}", elapsed);
                        total_records.to_owned()
                    },
                    Err(e) => {
                        println!("error : could not make http request for adx query : {:?}", e);
                        0 as u64
                    },
                };
                res
            }, // Ok
            Err(e) => {
                println!("error : could not fetch token : {:#?}", e);
                0
            }, // Err
        };
        total_records
    }

    fn get_azure_records(&self, azure_record: AzureRecord, top: i32, skip: i32) -> Vec<Value> {

        let my_scope = Some("https://management.azure.com/.default".to_owned());

        let empty_vec:Vec<Value> = Vec::new();

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(TokenType::AzureGraphToken, my_scope.to_owned());

        let api_url = resource_api_url.to_owned() + "/providers/Microsoft.ResourceGraph/resources?api-version=2020-04-01-preview";

        println!("api_url for azure_data_explorer : {}", api_url.as_str());

        let empty:Vec<String> = vec![];

        /*
            0 - 999      -> first page
            1000 - 1999  -> second page
            2000 - 2999  -> third page
            3000 - 3999  -> fourth page

            $top -> The maximum number of rows that the query should return. Overrides the page size when $skipToken property is present.
            $skip -> The number of rows to skip from the beginning of the results. Overrides the next page offset when $skipToken property is present.
        */

        let azure_entity = get_azure_record(azure_record);

        let query = format!("Resources | where type =~ '{}'", azure_entity);

        let json_payload = json!({
            "subscriptions": empty,
            "options" : {
                "$top" : top, // usually fixed at 1000
                "$skip" : skip
            },
            "query": query
        });

        println!("{}", serde_json::to_string_pretty(&json_payload).unwrap());

        let azure_data = match self.get_token(TokenType::AzureGraphToken, my_scope.to_owned()) {
            Ok(d) => {
                let client = reqwest::blocking::Client::new();
                let token = d.to_owned();
                let now = time::Instant::now();
                let res = match client.post(api_url.as_str()).body(json_payload.to_string()).header(CONTENT_TYPE, "application/json").header(AUTHORIZATION, token.as_str()).send() {
                    Ok(d) => {
                        let elapsed = now.elapsed();

                        println!("azure_data_explorer query result:");
                        let json_str = d.text().unwrap();
                        let json_data: Value = serde_json::from_str(json_str.as_str()).unwrap();
                        let rows = &json_data["data"]["rows"];

                        println!("TTT (top : {} , skip : {}) : {:.2?}", top, skip, elapsed);

                        let row_data = return match rows.as_array() {
                            None => {
                                println!("oops : could not find any row data !");
                                empty_vec
                            },
                            Some(d) => {
                                println!("found rows !");
                                d.to_vec()
                            },
                        };
                        row_data
                    },
                    Err(e) => {
                        println!("error : could not make http request for adx query : {:?}", e);
                        empty_vec
                    },
                };
                res.to_owned()
            },
            Err(e) => {
                println!("error : could not fetch token : {:#?}", e);
                empty_vec
            },
        };
        azure_data
    }

    fn get_all_azure_records(&self, azure_record: AzureRecord, records_per_page: i32, write_json_to_file: bool) -> Vec<Value> {
        let total_records_for_virtual_machines = self.get_total_records(azure_record);

        let total_pages = (total_records_for_virtual_machines as f32 / records_per_page as f32).ceil() as i32;

        println!("records_per_page : {}", records_per_page);
        println!("total_pages : {}", total_pages);

        let mut all_records: Vec<Value> = Vec::new();

        let now = time::Instant::now();

        let mut page_list: Vec<i32> = Vec::new();

        for i in 1..=total_pages {
            page_list.push(i);
        }

        let result: Vec<_> = page_list.chunks(8).collect();
        println!("{:#?}", result);

        for page_number in 1..=total_pages {
            println!("fetching data for page_number : {}...", page_number);

            let skip = get_skip_number(records_per_page, page_number);

            let records = self.get_azure_records(AzureRecord::VirtualMachine, records_per_page, skip);
            println!("total records for page_number {} : {}", page_number, records.len());
            for item in records.iter() {
                &all_records.push(item.to_owned());
            }
            println!("iteration in progress : length of all_records : {}", &all_records.len());
            let elapsed = now.elapsed();
            println!("total time taken so far : {:.2?}", elapsed);
        }

        println!("length of all_records : {}", &all_records.len());

        if write_json_to_file {
            let azure_entity = get_azure_record(azure_record);
            let output_file = azure_entity.replace("/", "_").replace(".", "_") + ".json";

            std::fs::write(
            output_file,
        serde_json::to_string_pretty(&all_records).unwrap(),
            ).unwrap();
        }
        all_records
    }
}

enum EnvironmentVarible {
    ClientID,
    ClientSecret,
    TenantID,
    Region,
}

fn get_env_var(env_var: EnvironmentVarible) -> String {
    return match env_var {
        EnvironmentVarible::ClientID => {
            match env::var("CLIENT_ID") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'CLIENT_ID' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::ClientSecret => {
            match env::var("CLIENT_SECRET") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'CLIENT_SECRET' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::TenantID => {
            match env::var("TENANT_ID") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'TENANT_ID' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::Region => {
            match env::var("REGION") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'TENANT_ID' is not set !");
                    "".to_owned()
                },
            }
        }
    };
}

fn main() {
    println!("Azure Data >");

    let client_id = get_env_var(EnvironmentVarible::ClientID);
    let client_secret = get_env_var(EnvironmentVarible::ClientSecret);
    let tenant_id = get_env_var(EnvironmentVarible::TenantID);
    let region = get_env_var(EnvironmentVarible::Region);

    let service_principal = ServicePrincipal {
        client_id,
        client_secret,
        region,
        tenant_id,
        token_type: TokenType::AzureApiToken,
    };

    // api token
    let api_token = service_principal.get_token(TokenType::AzureApiToken, None).unwrap();
    // graph token
    let graph_token = service_principal.get_token(TokenType::AzureGraphToken, None).unwrap();

    let all_recs = service_principal.get_all_azure_records(AzureRecord::VirtualMachine,1000,true);
    println!("all_recs.len() : {}", all_recs.len())

}

fn get_skip_number(records_per_page: i32, page_number: i32) -> i32 {
    (page_number-1) * records_per_page
}
```

Fetching Virtual Machines : Azure Resource Graph Query (ADX) , Using Cache & Multiple Threads : (FYI : Still, Work In Progress) : Part-4

```rust
use std::collections::HashMap;
use reqwest::{Response, StatusCode};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Map, Value};
use std::{env,time};
use std::borrow::Borrow;
use std::slice::Chunks;
use moka::sync::Cache;
use std::time::Duration;
use std::sync::mpsc;
use lazy_static::lazy_static;
use std::{thread};
use std::sync::mpsc::SendError;
// use crossbeam_utils::thread;
use crate::AzureRecord::VirtualMachine;


#[derive(Debug)]
pub enum GenericError {
    InvalidServicePrincipal,
    MissingDataInResponse,
}

#[derive(Debug)]
struct CustomError {
    err_type: GenericError,
    err_msg: String
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum TokenType {
    AzureApiToken,
    AzureGraphToken,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum AzureRecord {
    VirtualMachine
}

#[derive(Clone)]
struct ServicePrincipal {
    client_id: String,
    client_secret: String,
    tenant_id: String,
    region: String,
    token_type: TokenType
}

// global variable

lazy_static! {
    static ref APP_CACHE:Cache<TokenAndScope, String> = Cache::builder()
        // Time to live (TTL): 45 minutes
        .time_to_live(Duration::from_secs(45 * 60))
        // Time to idle (TTI):  45 minutes
        .time_to_idle(Duration::from_secs(45 * 60))
        // Create the cache.
        .build();

    static ref APP_CACHE_TOTAL_RECORDS:Cache<AzRecord, u64> = Cache::builder()
        // Time to live (TTL): 45 minutes
        .time_to_live(Duration::from_secs(45 * 60))
        // Time to idle (TTI):  45 minutes
        .time_to_idle(Duration::from_secs(45 * 60))
        // Create the cache.
        .build();
}

impl PartialEq<Self> for AzRecord {
    fn eq(&self, other: &Self) -> bool {
        self.azure_record == other.azure_record
    }
}

#[derive(Hash, Clone, Eq)]
struct AzRecord {
    azure_record: AzureRecord
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct TokenAndScope {
    token_type: TokenType,
    azure_scope: Option<String>
}

// microsoft public api endpoints

fn get_azure_region_details(region: &str) -> HashMap<String, String> {
    let mut azure_region: HashMap<String, String> = HashMap::new();

    return match region {
        "america" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        },
        "china" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.chinacloudapi.cn".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.partner.microsoftonline.cn".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://microsoftgraph.chinacloudapi.cn/.default".to_owned());
            azure_region
        },
        _ => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        }
    }
}

fn get_azure_record(azure_record: AzureRecord) -> String {
    return match azure_record {
        AzureRecord::VirtualMachine => "microsoft.compute/virtualmachines".to_owned(),
    }
}

impl ServicePrincipal {

    fn get_params_and_url(&self, token_type: TokenType, azure_scope: Option<String>) -> ([(String, String); 4], String, String, String) {
        let region = &self.region;
        let client_id = &self.client_id;
        let tenant_id = &self.tenant_id;
        let client_secret = &self.client_secret;

        let azure_region_details = get_azure_region_details(region.as_str());

        let authority_url = match token_type {
            TokenType::AzureApiToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/token";
                authority_url
            },
            TokenType::AzureGraphToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/v2.0/token";
                authority_url
            },
        };

        let resource_graph_url = azure_region_details.get("resourceGraphURL").unwrap().to_string();

        let resource_api_url = azure_region_details.get("resourceAPI").unwrap().to_string();

        // println!("authority_url      : {:?}", authority_url);
        // println!("resource_graph_url : {:?}", resource_graph_url);
        // println!("resource_api_url   : {:?}", resource_api_url);

        let my_scope = match azure_scope {
            None => {
                println!("azure_scope is None !");
                "https://graph.microsoft.com/.default".to_owned()
            },
            Some(d) => {
                d.to_owned()
            },
        };

        let params = match token_type {
            TokenType::AzureApiToken => {
                let params = [
                    ("grant_type".to_string(), "client_credentials".to_string()),
                    ("client_id".to_string(), client_id.to_string()),
                    ("client_secret".to_string(), client_secret.to_string()),
                    ("resource".to_string(), "".to_string()),
                ];
                params
            },
            TokenType::AzureGraphToken => {
                let params = [
                    ("grant_type".to_string(), "client_credentials".to_string()),
                    ("client_id".to_string(), client_id.to_string()),
                    ("client_secret".to_string(), client_secret.to_string()),
                    ("scope".to_string(), my_scope.to_string()),
                ];
                params
            }
        };
        let return_data = (params, authority_url, resource_graph_url, resource_api_url);
        // println!("{:#?}", &return_data);
        return return_data;
    }

    fn get_token(&self, token_type: TokenType, azure_scope: Option<String>) -> Result<String, CustomError> {

        let token_type_and_scope = TokenAndScope { token_type: token_type.to_owned(), azure_scope: azure_scope.to_owned() };

        let my_token = match APP_CACHE.get(&token_type_and_scope) {
            None => {
                println!("MISSING_TOKEN in APP_CACHE");
                "".to_owned()
            }
            Some(d) => {
                // println!("APP_CACHE : token : {}", d);
                d
            }
        };

        if my_token != "" {
            return Ok(my_token)
        }

        let region = &self.region;
        let client_id = &self.client_id;
        let client_secret = &self.client_secret;
        let region = &self.region;
        let tenant_id = &self.tenant_id;

        let my_scope = Some("https://management.azure.com/.default".to_string());

        let my_scope = match azure_scope {
            None => {
                println!("azure_scope is None !");
                "https://graph.microsoft.com/.default".to_owned()
            },
            Some(d) => {
                d.to_owned()
            },
        };

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(token_type, Some(my_scope));

        let client = reqwest::blocking::Client::new();

        let res = client.post(authority_url).form(&params).send();

        let result =  match res {
            Ok(res) => {
                println!("http response : {:?}", res);
                let data_str = res.text().unwrap_or("N/A".to_string());
                let value: Value = serde_json::from_str(data_str.as_str()).unwrap();

                let my_object = match value.as_object() {
                    None => {
                        println!("no object found");
                    },
                    Some(d) => {
                        if d.contains_key("token_type") == false || d.contains_key("access_token") == false {
                            let msg:String = format!("http response does not contains either the follow keys : (token_type|access_token)");
                            let custom_err = CustomError {
                                err_msg: String::from(msg),
                                err_type: GenericError::MissingDataInResponse
                            };
                            return Err(custom_err)
                        }
                    }
                };

                let token = format!("{} {}", &value["token_type"].as_str().unwrap(), &value["access_token"].as_str().unwrap());
                token
            },
            Err(err) => {
                let msg:String = format!("there was an error http form submit to get token {:?}", err);
                let custom_err = CustomError {
                    err_msg: String::from(msg),
                    err_type: GenericError::InvalidServicePrincipal
                };
                return Err(custom_err)
            }
        };
        println!("Storing api_token : {} , in APP_CACHE", result.to_owned());
        APP_CACHE.insert(token_type_and_scope, result.to_owned());
        Ok(result)
    }

    fn get_total_records(&self, azure_record: AzureRecord) -> u64 {

        let az_record = AzRecord { azure_record: azure_record.to_owned() };

        let total_records_from_cache = match APP_CACHE_TOTAL_RECORDS.get(&az_record) {
            None => {
                println!("__FUNC__ : get_total_records() : MISSING_TOTAL_RECORDS in APP_CACHE_TOTAL_RECORDS");
                0
            }
            Some(d) => {
                println!("__FUNC__ : get_total_records() : APP_CACHE_TOTAL_RECORDS : total_records_from_cache : {}", d);
                d
            }
        };

        if total_records_from_cache != 0 {
            println!("__FUNC__ : get_total_records() : total_records_from_cache : {}", total_records_from_cache);
            return total_records_from_cache
        }

        println!("__FUNC__ : data not found in cache !");

        //----------------------------------------------------------------------------------------

        let my_scope = Some("https://management.azure.com/.default".to_owned());

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(TokenType::AzureGraphToken, my_scope.to_owned());

        let api_url = resource_api_url.to_owned() + "/providers/Microsoft.ResourceGraph/resources?api-version=2020-04-01-preview";

        println!("api_url for azure_data_explorer : {}", api_url.as_str());

        let empty:Vec<String> = vec![];

        let record_name = get_azure_record(azure_record);

        let total_records_query = format!("Resources | where type == '{}' | summarize count()", record_name);

        let json_payload = json!({
            "subscriptions": empty,
            "query": total_records_query
        });

        println!("json_payload:");
        println!("{:#?}", json_payload);

        let total_records = match self.get_token(TokenType::AzureGraphToken, my_scope.to_owned()) {
            Ok(d) => {
                let client = reqwest::blocking::Client::new();
                let token = d.to_owned();
                println!("token for azure_data_explorer : {}", token.as_str());
                let now = time::Instant::now();
                let res = match client.post(api_url.as_str()).body(json_payload.to_string()).header(CONTENT_TYPE, "application/json").header(AUTHORIZATION, token.as_str()).send() {
                    Ok(d) => {
                        let elapsed = now.elapsed();
                        let json_str = d.text().unwrap();
                        let json_data: Value = serde_json::from_str(json_str.as_str()).unwrap();
                        println!("{}", serde_json::to_string_pretty(&json_data).unwrap());
                        let total_records = &json_data["data"]["rows"][0][0].as_u64().unwrap();
                        println!("total_records : {}", total_records);
                        println!("TTT : {:.2?}", elapsed);
                        total_records.to_owned()
                    },
                    Err(e) => {
                        println!("error : could not make http request for adx query : {:?}", e);
                        0 as u64
                    },
                };
                res
            }, // Ok
            Err(e) => {
                println!("error : could not fetch token : {:#?}", e);
                0
            }, // Err
        };
        APP_CACHE_TOTAL_RECORDS.insert(az_record, total_records.to_owned());
        println!("__FUNC__ : get_total_records() : total_records : {}", total_records_from_cache);
        total_records
    }

    fn get_azure_records(&self, azure_record: AzureRecord, top: i32, skip: i32) -> Vec<Value> {

        let my_scope = Some("https://management.azure.com/.default".to_owned());

        let empty_vec:Vec<Value> = Vec::new();

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(TokenType::AzureGraphToken, my_scope.to_owned());

        let api_url = resource_api_url.to_owned() + "/providers/Microsoft.ResourceGraph/resources?api-version=2020-04-01-preview";

        println!("api_url for azure_data_explorer : {}", api_url.as_str());

        let empty:Vec<String> = vec![];

        /*
            0 - 999      -> first page
            1000 - 1999  -> second page
            2000 - 2999  -> third page
            3000 - 3999  -> fourth page

            $top -> The maximum number of rows that the query should return. Overrides the page size when $skipToken property is present.
            $skip -> The number of rows to skip from the beginning of the results. Overrides the next page offset when $skipToken property is present.
        */

        let azure_entity = get_azure_record(azure_record);

        let query = format!("Resources | where type =~ '{}'", azure_entity);

        let json_payload = json!({
            "subscriptions": empty,
            "options" : {
                "$top" : top, // usually fixed at 1000
                "$skip" : skip
            },
            "query": query
        });

        println!("{}", serde_json::to_string_pretty(&json_payload).unwrap());

        let azure_data = match self.get_token(TokenType::AzureGraphToken, my_scope.to_owned()) {
            Ok(d) => {
                let client = reqwest::blocking::Client::new();
                let token = d.to_owned();
                let now = time::Instant::now();
                let res = match client.post(api_url.as_str()).body(json_payload.to_string()).header(CONTENT_TYPE, "application/json").header(AUTHORIZATION, token.as_str()).send() {
                    Ok(d) => {
                        let elapsed = now.elapsed();

                        println!("azure_data_explorer query result:");
                        let json_str = d.text().unwrap();
                        let json_data: Value = serde_json::from_str(json_str.as_str()).unwrap();
                        let rows = &json_data["data"]["rows"];

                        println!("TTT (top : {} , skip : {}) : {:.2?}", top, skip, elapsed);

                        let row_data = return match rows.as_array() {
                            None => {
                                println!("oops : could not find any row data !");
                                empty_vec
                            },
                            Some(d) => {
                                println!("found rows !");
                                d.to_vec()
                            },
                        };
                        row_data
                    },
                    Err(e) => {
                        println!("error : could not make http request for adx query : {:?}", e);
                        empty_vec
                    },
                };
                res.to_owned()
            },
            Err(e) => {
                println!("error : could not fetch token : {:#?}", e);
                empty_vec
            },
        };
        azure_data
    }

    fn get_total_pages_for_all_records(&self, azure_record: AzureRecord, records_per_page: i32) -> i32 {
        let total_records_for_virtual_machines = self.get_total_records(azure_record);
        let total_pages = (total_records_for_virtual_machines as f32 / records_per_page as f32).ceil() as i32;
        println!("__FUNC__ : get_total_pages_for_all_records() : {}", total_pages);
        total_pages
    }

    fn get_all_azure_records_for_page(&self, page_number: i32, azure_record: AzureRecord, records_per_page: i32) -> Vec<Value> {
        let total_pages = self.get_total_pages_for_all_records(azure_record, records_per_page);
        let mut all_records: Vec<Value> = Vec::new();
        let now = time::Instant::now();
        println!("__FUNC__: get_all_azure_records_for_page() : fetching data for page_number : {}...", page_number);
        let skip = get_skip_number(records_per_page, page_number);
        let records = self.get_azure_records(AzureRecord::VirtualMachine, records_per_page, skip);
        println!("total records for page_number {} : {}", page_number, records.len());
        for item in records.iter() {
            &all_records.push(item.to_owned());
        }
        let elapsed = now.elapsed();
        println!("__FUNC__ : get_all_azure_records_for_page() : total time taken for page_number {} : {:.2?}", page_number, elapsed);
        println!("__FUNC__ : get_all_azure_records_for_page() : all_records.len() : {}", &all_records.len());
        all_records
    }

    fn get_all_azure_records_for_pages(&self, page_numbers: Vec<i32>, azure_record: AzureRecord, records_per_page: i32) -> Vec<Value> {
        let total_pages = self.get_total_pages_for_all_records(azure_record, records_per_page);
        let mut all_records: Vec<Value> = Vec::new();
        let now = time::Instant::now();

        for page_number in page_numbers {
            println!("__FUNC__ : get_all_azure_records_for_pages() : fetching data for page_number : {}...", page_number);
            let skip = get_skip_number(records_per_page, page_number);
            let records = self.get_azure_records(AzureRecord::VirtualMachine, records_per_page, skip);

            println!("__FUNC__ : get_all_azure_records_for_pages() : total records for page_number {} : {}", page_number, records.len());
            for item in records.iter() {
                &all_records.push(item.to_owned());
            }
            let elapsed = now.elapsed();
            println!("__FUNC__ : get_all_azure_records_for_pages() : total time taken so far : {:.2?}", elapsed);
        }
        println!("__FUNC__ : get_all_azure_records_for_pages() : length of all_records : {}", &all_records.len());
        all_records
    }

    fn get_all_azure_records(&self, azure_record: AzureRecord, records_per_page: i32, write_json_to_file: bool) -> Vec<Value> {

        let total_pages = self.get_total_pages_for_all_records(azure_record, records_per_page);

        let mut all_records: Vec<Value> = Vec::new();

        let now = time::Instant::now();

        for page_number in 1..=total_pages {
            let records = self.get_all_azure_records_for_page(page_number, azure_record, records_per_page);
            println!("total records for page_number {} : {}", page_number, records.len());
            for item in records.iter() {
                &all_records.push(item.to_owned());
            }
            let elapsed = now.elapsed();
            println!("__FUNC__ : get_all_azure_records() : total time taken so far : {:.2?}", elapsed);
        }

        println!("__FUNC__ : get_all_azure_records() : length of all_records : {}", &all_records.len());

        if write_json_to_file {
            let azure_entity = get_azure_record(azure_record);
            let output_file = azure_entity.replace("/", "_").replace(".", "_") + ".json";

            std::fs::write(
            output_file,
        serde_json::to_string_pretty(&all_records).unwrap(),
            ).unwrap();
        }
        all_records
    }
}

enum EnvironmentVarible {
    ClientID,
    ClientSecret,
    TenantID,
    Region,
}

fn get_env_var(env_var: EnvironmentVarible) -> String {
    return match env_var {
        EnvironmentVarible::ClientID => {
            match env::var("CLIENT_ID") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'CLIENT_ID' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::ClientSecret => {
            match env::var("CLIENT_SECRET") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'CLIENT_SECRET' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::TenantID => {
            match env::var("TENANT_ID") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'TENANT_ID' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::Region => {
            match env::var("REGION") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'TENANT_ID' is not set !");
                    "".to_owned()
                },
            }
        }
    };
}

fn main() {
    println!("Azure Data >");

    let client_id = get_env_var(EnvironmentVarible::ClientID);
    let client_secret = get_env_var(EnvironmentVarible::ClientSecret);
    let tenant_id = get_env_var(EnvironmentVarible::TenantID);
    let region = get_env_var(EnvironmentVarible::Region);

    let mut service_principal = ServicePrincipal {
        client_id,
        client_secret,
        region,
        tenant_id,
        token_type: TokenType::AzureApiToken,
    };

    // api token
    let api_token = service_principal.get_token(TokenType::AzureApiToken, None).unwrap();
    // graph token
    let graph_token = service_principal.get_token(TokenType::AzureGraphToken, None).unwrap();

    let all_recs = get_all_azure_records_in_parallel(&service_principal, AzureRecord::VirtualMachine, 1000, true);

}

fn get_skip_number(records_per_page: i32, page_number: i32) -> i32 {
    (page_number-1) * records_per_page
}


fn get_all_azure_records_in_parallel(service_principal: &ServicePrincipal, azure_record: AzureRecord, records_per_page: i32, write_json_to_file: bool) -> Vec<Value> {

    println!("__FUNC__ : get_all_azure_records_in_parallel() ...");

    let now = time::Instant::now();

    let total_pages = service_principal.get_total_pages_for_all_records(azure_record, records_per_page);

    let mut page_numbers: Vec<i32> = Vec::new();

    for i in 1..=total_pages {
        page_numbers.push(i);
    }

    let (sender, receiver) = mpsc::channel();

    // let result_pages: Vec<_> = page_numbers.chunks(8).collect();

    let v_chunked: Vec<Vec<i32>> = page_numbers.chunks(8).map(|x| x.to_vec()).collect();

    println!("v_chunked : {:?}", v_chunked);

    let mut all_records: Vec<Value> = Vec::new();

    // let mut threads = vec![];

    let total_count_of_chunks_of_vectors = v_chunked.len();

    for page_numbers in v_chunked {

        let sender = sender.clone();

        // let spn = service_principal.clone();

        let spn = service_principal.to_owned();

        thread::spawn(move || {

            let records = spn.get_all_azure_records_for_pages(page_numbers, azure_record, records_per_page);

            println!("__VEC_LENGTH__ : {}", records.len());

            let send_data = match sender.send(records) {
                Ok(d) => {
                    println!("__FUNC__ : get_all_azure_records_in_parallel() : data was sent from sender to receiver.")
                },
                Err(e) => {
                    println!("__FUNC__ : error : get_all_azure_records_in_parallel() : data send failed : {:#?}", e);
                }
            };

        });
    }

    drop(sender);

    for _ in 0..total_count_of_chunks_of_vectors {
        let records = receiver.recv().unwrap();

        println!("__FUNC__ : _received {}  records from (receiver) ", records.len());
        for rec in records.iter() {
            &all_records.push(rec.to_owned());
        }
        println!("__FUNC__ : _total Length of all_records {}", all_records.len());
    }

    println!("__FUNC__ : _final : length of all_records : {}", &all_records.len());


    if write_json_to_file {
        let azure_entity = get_azure_record(azure_record);
        let output_file = azure_entity.replace("/", "_").replace(".", "_") + ".json";

        std::fs::write(
        output_file,
    serde_json::to_string_pretty(&all_records).unwrap(),
        ).unwrap();
    }

    let elapsed = now.elapsed();

    println!("__FUNC__ : get_all_azure_records_in_parallel() : final_time : {:.2?}", elapsed);

    all_records
}
```

Fetching Virtual Machines : Azure Resource Graph Query (ADX) , Using Cache & Multiple Threads : With BQ Inserts : Part-5

`Cargo.toml`

```toml
[package]
name = "azure-rust"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.5"
regex = "1.7.0"
reqwest = { version = "0.11.13" , features = ["blocking"] } # reqwest with JSON parsing support
#reqwest = { version = "0.11.13" } # reqwest with JSON parsing support
tokio = { version = "1.23.0", features = ["full"] }
serde = "1.0.152"
serde_json = "1.0.91"
serde_derive = "1.0.152"
rayon = "1.6.1"
futures = "0.3.4" # for our async / await blocks
moka = "0.9.6"
lazy_static = "1.4.0"
quick_cache = "0.2.0"
cached = "0.42.0"
crossbeam-utils = "0.8.14"
reqwest-retry = "0.2.1"
reqwest-middleware = "0.2.0"
gcp-bigquery-client = "0.16.5"
chrono = "0.4.23"
```

Export Environment Variables

```bash
export GOOGLE_APPLICATION_CREDENTIALS="$HOME/svc-account.json"
export PROJECT_ID="<PRIJECT_ID>"
export DATASET_ID="<GCP_DATASET>"
export TABLE_ID="<TABLE_NAME>"
```

For this code to work, on GCP BigQuery table we have

```bash
- id       -> column type : STRING
- metadata -> column type : JSON
- datetime -> column type : DATETIME
- entity   -> column type : STRING
```

`src/main.rs`

```rust
use std::collections::HashMap;
use reqwest::{ClientBuilder, Response, StatusCode};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Map, Value};
use serde::Serialize;

use chrono;
use std::{env,time};
use std::borrow::Borrow;
use std::slice::Chunks;
use moka::sync::Cache;
use std::time::Duration;
use std::sync::mpsc;
use lazy_static::lazy_static;
use std::{thread};
use std::sync::mpsc::SendError;
use gcp_bigquery_client::Client;
use gcp_bigquery_client::model::table::Table;
use gcp_bigquery_client::model::table_data_insert_all_request::TableDataInsertAllRequest;
use gcp_bigquery_client::model::table_data_insert_all_response::TableDataInsertAllResponse;
use gcp_bigquery_client::model::table_field_schema::TableFieldSchema;
use gcp_bigquery_client::model::table_schema::TableSchema;
// use crossbeam_utils::thread;
use crate::AzureRecord::VirtualMachine;

use reqwest_middleware::{ClientWithMiddleware};
use reqwest_middleware::ClientBuilder as ReqwestClientBuilder;
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use serde_derive::Deserialize;

use chrono::prelude::*;

#[derive(Debug)]
pub enum GenericError {
    InvalidServicePrincipal,
    MissingDataInResponse,
}

#[derive(Debug)]
struct CustomError {
    err_type: GenericError,
    err_msg: String
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum TokenType {
    AzureApiToken,
    AzureGraphToken,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum AzureRecord {
    VirtualMachine
}

#[derive(Clone)]
struct ServicePrincipal {
    client_id: String,
    client_secret: String,
    tenant_id: String,
    region: String,
    token_type: TokenType
}

// global variable

lazy_static! {
    static ref APP_CACHE:Cache<TokenAndScope, String> = Cache::builder()
        // Time to live (TTL): 45 minutes
        .time_to_live(Duration::from_secs(45 * 60))
        // Time to idle (TTI):  45 minutes
        .time_to_idle(Duration::from_secs(45 * 60))
        // Create the cache.
        .build();

    static ref APP_CACHE_TOTAL_RECORDS:Cache<AzRecord, u64> = Cache::builder()
        // Time to live (TTL): 45 minutes
        .time_to_live(Duration::from_secs(45 * 60))
        // Time to idle (TTI):  45 minutes
        .time_to_idle(Duration::from_secs(45 * 60))
        // Create the cache.
        .build();
}

impl PartialEq<Self> for AzRecord {
    fn eq(&self, other: &Self) -> bool {
        self.azure_record == other.azure_record
    }
}

#[derive(Hash, Clone, Eq)]
struct AzRecord {
    azure_record: AzureRecord
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct TokenAndScope {
    token_type: TokenType,
    azure_scope: Option<String>
}

// microsoft public api endpoints

fn get_azure_region_details(region: &str) -> HashMap<String, String> {
    let mut azure_region: HashMap<String, String> = HashMap::new();

    return match region {
        "america" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        },
        "china" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.chinacloudapi.cn".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.partner.microsoftonline.cn".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://microsoftgraph.chinacloudapi.cn/.default".to_owned());
            azure_region
        },
        _ => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        }
    }
}

fn get_azure_record(azure_record: AzureRecord) -> String {
    return match azure_record {
        AzureRecord::VirtualMachine => "microsoft.compute/virtualmachines".to_owned(),
    }
}

impl ServicePrincipal {

    fn get_params_and_url(&self, token_type: TokenType, azure_scope: Option<String>) -> ([(String, String); 4], String, String, String) {
        let region = &self.region;
        let client_id = &self.client_id;
        let tenant_id = &self.tenant_id;
        let client_secret = &self.client_secret;

        let azure_region_details = get_azure_region_details(region.as_str());

        let authority_url = match token_type {
            TokenType::AzureApiToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/token";
                authority_url
            },
            TokenType::AzureGraphToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/v2.0/token";
                authority_url
            },
        };

        let resource_graph_url = azure_region_details.get("resourceGraphURL").unwrap().to_string();

        let resource_api_url = azure_region_details.get("resourceAPI").unwrap().to_string();

        let my_scope = match azure_scope {
            None => {
                println!("azure_scope is None !");
                "https://graph.microsoft.com/.default".to_owned()
            },
            Some(d) => {
                d.to_owned()
            },
        };

        let params = match token_type {
            TokenType::AzureApiToken => {
                let params = [
                    ("grant_type".to_string(), "client_credentials".to_string()),
                    ("client_id".to_string(), client_id.to_string()),
                    ("client_secret".to_string(), client_secret.to_string()),
                    ("resource".to_string(), "".to_string()),
                ];
                params
            },
            TokenType::AzureGraphToken => {
                let params = [
                    ("grant_type".to_string(), "client_credentials".to_string()),
                    ("client_id".to_string(), client_id.to_string()),
                    ("client_secret".to_string(), client_secret.to_string()),
                    ("scope".to_string(), my_scope.to_string()),
                ];
                params
            }
        };
        let return_data = (params, authority_url, resource_graph_url, resource_api_url);
        // println!("{:#?}", &return_data);
        return return_data;
    }

    fn get_token(&self, token_type: TokenType, azure_scope: Option<String>) -> Result<String, CustomError> {

        let token_type_and_scope = TokenAndScope { token_type: token_type.to_owned(), azure_scope: azure_scope.to_owned() };

        let my_token = match APP_CACHE.get(&token_type_and_scope) {
            None => {
                println!("MISSING_TOKEN in APP_CACHE");
                "".to_owned()
            }
            Some(d) => {
                // println!("APP_CACHE : token : {}", d);
                d
            }
        };

        if my_token != "" {
            return Ok(my_token)
        }

        let region = &self.region;
        let client_id = &self.client_id;
        let client_secret = &self.client_secret;
        let region = &self.region;
        let tenant_id = &self.tenant_id;

        let my_scope = Some("https://management.azure.com/.default".to_string());

        let my_scope = match azure_scope {
            None => {
                println!("azure_scope is None !");
                "https://graph.microsoft.com/.default".to_owned()
            },
            Some(d) => {
                d.to_owned()
            },
        };

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(token_type, Some(my_scope));

        let client = reqwest::blocking::Client::new();

        let res = client.post(authority_url).form(&params).send();

        let result =  match res {
            Ok(res) => {
                println!("http response : {:?}", res);
                let data_str = res.text().unwrap_or("N/A".to_string());
                let value: Value = serde_json::from_str(data_str.as_str()).unwrap();

                let my_object = match value.as_object() {
                    None => {
                        println!("no object found");
                    },
                    Some(d) => {
                        if d.contains_key("token_type") == false || d.contains_key("access_token") == false {
                            let msg:String = format!("http response does not contains either the follow keys : (token_type|access_token)");
                            let custom_err = CustomError {
                                err_msg: String::from(msg),
                                err_type: GenericError::MissingDataInResponse
                            };
                            return Err(custom_err)
                        }
                    }
                };

                let token = format!("{} {}", &value["token_type"].as_str().unwrap(), &value["access_token"].as_str().unwrap());
                token
            },
            Err(err) => {
                let msg:String = format!("there was an error http form submit to get token {:?}", err);
                let custom_err = CustomError {
                    err_msg: String::from(msg),
                    err_type: GenericError::InvalidServicePrincipal
                };
                return Err(custom_err)
            }
        };
        println!("Storing api_token : {} , in APP_CACHE", result.to_owned());
        APP_CACHE.insert(token_type_and_scope, result.to_owned());
        Ok(result)
    }

    fn get_total_records(&self, azure_record: AzureRecord) -> u64 {

        let az_record = AzRecord { azure_record: azure_record.to_owned() };

        let total_records_from_cache = match APP_CACHE_TOTAL_RECORDS.get(&az_record) {
            None => {
                println!("__FUNC__ : get_total_records() : MISSING_TOTAL_RECORDS in APP_CACHE_TOTAL_RECORDS");
                0
            }
            Some(d) => {
                println!("__FUNC__ : get_total_records() : APP_CACHE_TOTAL_RECORDS : total_records_from_cache : {}", d);
                d
            }
        };

        if total_records_from_cache != 0 {
            println!("__FUNC__ : get_total_records() : total_records_from_cache : {}", total_records_from_cache);
            return total_records_from_cache
        }

        println!("__FUNC__ : data not found in cache !");

        //----------------------------------------------------------------------------------------

        let my_scope = Some("https://management.azure.com/.default".to_owned());

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(TokenType::AzureGraphToken, my_scope.to_owned());

        let api_url = resource_api_url.to_owned() + "/providers/Microsoft.ResourceGraph/resources?api-version=2020-04-01-preview";

        println!("api_url for azure_data_explorer : {}", api_url.as_str());

        let empty:Vec<String> = vec![];

        let record_name = get_azure_record(azure_record);

        let total_records_query = format!("Resources | where type == '{}' | summarize count()", record_name);

        let json_payload = json!({
            "subscriptions": empty,
            "query": total_records_query
        });

        println!("json_payload:");
        println!("{:#?}", json_payload);

        let total_records = match self.get_token(TokenType::AzureGraphToken, my_scope.to_owned()) {
            Ok(d) => {
                let client = reqwest::blocking::Client::new();
                let token = d.to_owned();
                println!("token for azure_data_explorer : {}", token.as_str());
                let now = time::Instant::now();
                let res = match client.post(api_url.as_str()).body(json_payload.to_string()).header(CONTENT_TYPE, "application/json").header(AUTHORIZATION, token.as_str()).send() {
                    Ok(d) => {
                        let elapsed = now.elapsed();
                        let json_str = d.text().unwrap();
                        let json_data: Value = serde_json::from_str(json_str.as_str()).unwrap();
                        println!("{}", serde_json::to_string_pretty(&json_data).unwrap());
                        let total_records = &json_data["data"]["rows"][0][0].as_u64().unwrap();
                        println!("total_records : {}", total_records);
                        println!("TTT : {:.2?}", elapsed);
                        total_records.to_owned()
                    },
                    Err(e) => {
                        println!("error : could not make http request for adx query : {:?}", e);
                        0 as u64
                    },
                };
                res
            }, // Ok
            Err(e) => {
                println!("error : could not fetch token : {:#?}", e);
                0
            }, // Err
        };
        APP_CACHE_TOTAL_RECORDS.insert(az_record, total_records.to_owned());
        println!("__FUNC__ : get_total_records() : total_records : {}", total_records_from_cache);
        total_records
    }

    fn get_azure_records(&self, azure_record: AzureRecord, top: i32, skip: i32) -> Vec<Value> {

        let my_scope = Some("https://management.azure.com/.default".to_owned());

        let empty_vec:Vec<Value> = Vec::new();

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(TokenType::AzureGraphToken, my_scope.to_owned());

        let api_url = resource_api_url.to_owned() + "/providers/Microsoft.ResourceGraph/resources?api-version=2020-04-01-preview";

        println!("api_url for azure_data_explorer : {}", api_url.as_str());

        let empty:Vec<String> = vec![];

        /*
            0 - 999      -> first page
            1000 - 1999  -> second page
            2000 - 2999  -> third page
            3000 - 3999  -> fourth page

            $top -> The maximum number of rows that the query should return. Overrides the page size when $skipToken property is present.
            $skip -> The number of rows to skip from the beginning of the results. Overrides the next page offset when $skipToken property is present.
        */

        let azure_entity = get_azure_record(azure_record);

        let query = format!("Resources | where type =~ '{}'", azure_entity);

        let json_payload = json!({
            "subscriptions": empty,
            "options" : {
                "$top" : top, // usually fixed at 1000
                "$skip" : skip
            },
            "query": query
        });

        println!("{}", serde_json::to_string_pretty(&json_payload).unwrap());

        let azure_data = match self.get_token(TokenType::AzureGraphToken, my_scope.to_owned()) {
            Ok(d) => {

                let client = reqwest::blocking::Client::new();

                let token = d.to_owned();
                let now = time::Instant::now();

                let res = match client.post(api_url.as_str()).body(json_payload.to_string()).header(CONTENT_TYPE, "application/json").header(AUTHORIZATION, token.as_str()).send() {
                    Ok(d) => {
                        let elapsed = now.elapsed();

                        println!("azure_data_explorer query result:");
                        let json_str = d.text().unwrap();
                        let json_data: Value = serde_json::from_str(json_str.as_str()).unwrap();
                        let rows = &json_data["data"]["rows"];

                        println!("TTT (top : {} , skip : {}) : {:.2?}", top, skip, elapsed);

                        let row_data = match rows.as_array() {
                            None => {
                                println!("oops : could not find any row data !");
                                empty_vec
                            },
                            Some(d) => {
                                println!("found rows !");
                                d.to_vec()
                            },
                        };
                        row_data
                    },
                    Err(e) => {
                        println!("error : could not make http request for adx query : {:?}", e);
                        empty_vec
                    },
                };
                res.to_owned()
            },
            Err(e) => {
                println!("error : could not fetch token : {:#?}", e);
                empty_vec
            },
        };
        azure_data
    }

    fn get_total_pages_for_all_records(&self, azure_record: AzureRecord, records_per_page: i32) -> i32 {
        let total_records_for_virtual_machines = self.get_total_records(azure_record);
        let total_pages = (total_records_for_virtual_machines as f32 / records_per_page as f32).ceil() as i32;
        println!("__FUNC__ : get_total_pages_for_all_records() : {}", total_pages);
        total_pages
    }

    fn get_all_azure_records_for_page(&self, page_number: i32, azure_record: AzureRecord, records_per_page: i32) -> Vec<Value> {
        let total_pages = self.get_total_pages_for_all_records(azure_record, records_per_page);
        let mut all_records: Vec<Value> = Vec::new();
        let now = time::Instant::now();
        println!("__FUNC__: get_all_azure_records_for_page() : fetching data for page_number : {}...", page_number);
        let skip = get_skip_number(records_per_page, page_number);
        let records = self.get_azure_records(AzureRecord::VirtualMachine, records_per_page, skip);
        println!("total records for page_number {} : {}", page_number, records.len());
        for item in records.iter() {
            &all_records.push(item.to_owned());
        }
        let elapsed = now.elapsed();
        println!("__FUNC__ : get_all_azure_records_for_page() : total time taken for page_number {} : {:.2?}", page_number, elapsed);
        println!("__FUNC__ : get_all_azure_records_for_page() : all_records.len() : {}", &all_records.len());
        all_records
    }

    fn get_all_azure_records_for_pages(&self, page_numbers: Vec<i32>, azure_record: AzureRecord, records_per_page: i32) -> Vec<Value> {
        let total_pages = self.get_total_pages_for_all_records(azure_record, records_per_page);
        let mut all_records: Vec<Value> = Vec::new();
        let now = time::Instant::now();

        for page_number in page_numbers {
            println!("__FUNC__ : get_all_azure_records_for_pages() : fetching data for page_number : {}...", page_number);
            let skip = get_skip_number(records_per_page, page_number);
            let records = self.get_azure_records(AzureRecord::VirtualMachine, records_per_page, skip);

            println!("__FUNC__ : get_all_azure_records_for_pages() : total records for page_number {} : {}", page_number, records.len());
            for item in records.iter() {
                &all_records.push(item.to_owned());
            }
            let elapsed = now.elapsed();
            println!("__FUNC__ : get_all_azure_records_for_pages() : total time taken so far : {:.2?}", elapsed);
        }
        println!("__FUNC__ : get_all_azure_records_for_pages() : length of all_records : {}", &all_records.len());
        all_records
    }

    fn get_all_azure_records(&self, azure_record: AzureRecord, records_per_page: i32, write_json_to_file: bool) -> Vec<Value> {

        let total_pages = self.get_total_pages_for_all_records(azure_record, records_per_page);

        let mut all_records: Vec<Value> = Vec::new();

        let now = time::Instant::now();

        for page_number in 1..=total_pages {
            let records = self.get_all_azure_records_for_page(page_number, azure_record, records_per_page);
            println!("total records for page_number {} : {}", page_number, records.len());
            for item in records.iter() {
                &all_records.push(item.to_owned());
            }
            let elapsed = now.elapsed();
            println!("__FUNC__ : get_all_azure_records() : total time taken so far : {:.2?}", elapsed);
        }

        println!("__FUNC__ : get_all_azure_records() : length of all_records : {}", &all_records.len());

        if write_json_to_file {
            write_vec_of_values_to_file("".to_string(), &all_records, Some(azure_record));
        }
        all_records
    }
}

enum EnvironmentVarible {
    ClientID,
    ClientSecret,
    TenantID,
    Region,
}

fn get_env_var(env_var: EnvironmentVarible) -> String {
    return match env_var {
        EnvironmentVarible::ClientID => {
            match env::var("CLIENT_ID") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'CLIENT_ID' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::ClientSecret => {
            match env::var("CLIENT_SECRET") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'CLIENT_SECRET' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::TenantID => {
            match env::var("TENANT_ID") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'TENANT_ID' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::Region => {
            match env::var("REGION") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'TENANT_ID' is not set !");
                    "".to_owned()
                },
            }
        }
    };
}

#[derive(Serialize, Debug, Clone)]
struct Metadata {
    id: String,
    metadata: String,
    datetime: String,
    entity: String
}

fn main() {
    println!("Azure Data >");

    let client_id = get_env_var(EnvironmentVarible::ClientID);
    let client_secret = get_env_var(EnvironmentVarible::ClientSecret);
    let tenant_id = get_env_var(EnvironmentVarible::TenantID);
    let region = get_env_var(EnvironmentVarible::Region);

    let mut service_principal = ServicePrincipal {
        client_id,
        client_secret,
        region,
        tenant_id,
        token_type: TokenType::AzureApiToken,
    };

    // api token
    let api_token = service_principal.get_token(TokenType::AzureApiToken, None).unwrap();

    // graph token
    let graph_token = service_principal.get_token(TokenType::AzureGraphToken, None).unwrap();

    let now_az_query = time::Instant::now();

    let all_recs = get_all_azure_records_in_parallel(&service_principal, AzureRecord::VirtualMachine, 1000, true);

    let elapsed_az_query = now_az_query.elapsed();

    println!("__FUNC__ : total time for azure query : {:.2?}", elapsed_az_query);

    perform_bigquery_inserts_for_azure_record(AzureRecord::VirtualMachine, &all_recs, "<MY_GCP_PROJECT_NAME>".to_string(), "<MY_BQ_DATASET_NAME>".to_string(), "<MY_BQ_TABLE>".to_string())
}

#[tokio::main]
async fn perform_bigquery_inserts_for_azure_record(azure_record: AzureRecord, all_recs: &Vec<Value>, project_id: String, dataset_id: String, table_id: String) {
    let now = Utc::now();
    let ts: i64 = now.timestamp();
    let nt = NaiveDateTime::from_timestamp_opt(ts, 0).unwrap();
    let dt: DateTime<Utc> = DateTime::from_utc(nt, Utc);
    let current_time_stamp = dt.format("%Y-%m-%d %H:%M:%S").to_string();

    let azure_entity = get_azure_record(azure_record);

    let gcp_client = get_google_client().await.unwrap();

    let bq_data_set = gcp_client.dataset().get(project_id.as_str(), dataset_id.as_str()).await.unwrap();

    let mut vec_bq_insert: Vec<Metadata> = Vec::new();

    for record in all_recs.iter() {
        let rec_id = record[0].as_str().unwrap();
        let rec_metadata = serde_json::to_string(record).unwrap();
        let metadata = Metadata {
            id: rec_id.to_owned(),
            metadata: rec_metadata,
            datetime: current_time_stamp.to_string(),
            entity: azure_entity.to_string(),
        };
        vec_bq_insert.push(metadata);
    }

    let v_chunks: Vec<Vec<Metadata>> = vec_bq_insert.chunks(500).map(|x| x.to_vec()).collect();

    let total_chunks = v_chunks.len();

    let now_bq_insert = time::Instant::now();

    let mut chunk_counter = 1;

    for chunks in v_chunks {

        println!("chunk_counter : {} / {}", chunk_counter, total_chunks);

        let mut insert_request = TableDataInsertAllRequest::new();

        for chunk in chunks.iter() {
            insert_request.add_row(
                None,
                chunk
            ).expect("could not add row");
        }
        let res = gcp_client.tabledata().insert_all(project_id.as_str(), dataset_id.as_str(), table_id.as_str(), insert_request).await;

        println!("res : {:#?}", res);

        chunk_counter = chunk_counter + 1;
    }

    let elapsed_bq_insert = now_bq_insert.elapsed();

    println!("__FUNC__ : perform_bigquery_inserts_for_azure_record() : total time for big_query inserts : {:.2?}", elapsed_bq_insert);
}

async fn get_google_client() -> Result<Client, Box<dyn std::error::Error>>{
    let gcp_sa_key = match env::var("GOOGLE_APPLICATION_CREDENTIALS") {
        Ok(v) => v,
        Err(e) => {
            println!("environment variable 'GOOGLE_APPLICATION_CREDENTIALS' is not set !");
            "".to_owned()
        },
    };

    if gcp_sa_key == "" {
        let msg = format!("error : could not get file path for environment variable 'GOOGLE_APPLICATION_CREDENTIALS'");
        return Err(Box::try_from(msg).unwrap())
    }

    let gcp_client = gcp_bigquery_client::Client::from_service_account_key_file(gcp_sa_key.as_str()).await.unwrap();
    Ok(gcp_client)
}

fn get_skip_number(records_per_page: i32, page_number: i32) -> i32 {
    (page_number-1) * records_per_page
}


fn get_all_azure_records_in_parallel(service_principal: &ServicePrincipal, azure_record: AzureRecord, records_per_page: i32, write_json_to_file: bool) -> Vec<Value> {

    println!("__FUNC__ : get_all_azure_records_in_parallel() ...");

    let now = time::Instant::now();

    let total_pages = service_principal.get_total_pages_for_all_records(azure_record, records_per_page);

    let mut page_numbers: Vec<i32> = Vec::new();

    for i in 1..=total_pages {
        page_numbers.push(i);
    }

    let (sender, receiver) = mpsc::channel();

    let v_chunked: Vec<Vec<i32>> = page_numbers.chunks(8).map(|x| x.to_vec()).collect();

    println!("v_chunked : {:?}", v_chunked);

    let mut all_records: Vec<Value> = Vec::new();

    let total_count_of_chunks_of_vectors = v_chunked.len();

    for page_numbers in v_chunked {

        let sender = sender.clone();

        let spn = service_principal.to_owned();

        thread::spawn(move || {

            let records = spn.get_all_azure_records_for_pages(page_numbers, azure_record, records_per_page);

            println!("__VEC_LENGTH__ : {}", records.len());

            let send_data = match sender.send(records) {
                Ok(d) => {
                    println!("__FUNC__ : get_all_azure_records_in_parallel() : data was sent from sender to receiver.")
                },
                Err(e) => {
                    println!("__FUNC__ : error : get_all_azure_records_in_parallel() : data send failed : {:#?}", e);
                }
            };

        });
    }

    drop(sender);

    for _ in 0..total_count_of_chunks_of_vectors {
        let records = receiver.recv().unwrap();

        println!("__FUNC__ : _received {}  records from (receiver) ", records.len());
        for rec in records.iter() {
            &all_records.push(rec.to_owned());
        }
        println!("__FUNC__ : _total Length of all_records {}", all_records.len());
    }

    println!("__FUNC__ : _final : length of all_records : {}", &all_records.len());

    write_vec_of_values_to_file("".to_string(), &all_records, Some(azure_record));

    let elapsed = now.elapsed();

    println!("__FUNC__ : get_all_azure_records_in_parallel() : final_time : {:.2?}", elapsed);

    all_records
}

fn write_vec_of_values_to_file(mut output_file: String, all_records: &Vec<Value>, azure_record: Option<AzureRecord>) {
    let out_file_for_azure_record = match azure_record {
        None => {
            "".to_owned()
        },
        Some(d) => {
            let azure_entity = get_azure_record(d);
            let output_file = azure_entity.replace("/", "_").replace(".", "_") + ".json";
            output_file
        },
    };
    if out_file_for_azure_record != "" {
        output_file = out_file_for_azure_record;
    }

    println!("json file ( {} ) written !", output_file);

    std::fs::write(output_file, serde_json::to_string_pretty(&all_records).unwrap()).unwrap();
}
```