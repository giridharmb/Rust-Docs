## Rust & jQuery Data Table

Files
- src/main.rs
- templates/index.html
- app.rust.env

> Prepare PostgreSQL DB/Table >

```sql
CREATE TABLE t_random(
   random_num INT NOT NULL,
   random_float DOUBLE PRECISION NOT NULL,
   md5 TEXT NOT NULL
);

INSERT INTO t_random (random_num, random_float, md5) 
    SELECT 
    floor(random()* (999-100 + 1) + 100), 
    random(), 
    md5(random()::text) 
 from generate_series(1,100);
```

> Backend PostgreSQL Table Structure >

```sql
\d t_random;
                     Table "public.t_random"
    Column    |       Type       | Collation | Nullable | Default
--------------+------------------+-----------+----------+---------
 random_num   | integer          |           | not null |
 random_float | double precision |           | not null |
 md5          | text             |           | not null |


select * from t_random limit 5;

 random_num |    random_float     |               md5
------------+---------------------+----------------------------------
        349 |  0.7457606724537023 | 14569004fc1e3156ef5c2b95617863b6
        361 |  0.5383973494637289 | e06d3d8ca3b7256f7e861f6e007f0295
        870 | 0.31454480989041755 | 54160809bc82e4eda0f4cd69ba979cc4
        372 | 0.16725864580014616 | 76375027264b59b82beaede6ef929a26
        598 |  0.9532303436714109 | 11c50afd37d5df70dbff23618aa01d11
(5 rows)
```

How To Run The Program >
- `cargo run`
- Access the web URL : http://localhost:5050/tables

FYI : `Cargo.toml` has some extra dependencies that are not needed

`Cargo.toml`

```toml
[package]
name = "rust-datatable"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
actix-rt = "2.8.0"
env_logger = "0.10.0"
actix-cors = "0.6.4"
actix-web = "4.3.0"
serde_json = "1.0.93"
serde = { version = "1.0.2" , features = ["derive"] }
reqwest = { version = "0.11.18" , features = ["json"] }
serde_yaml = "0.9.25"
serde_derive = "1.0.176"
tokio = { version = "1.29.1", features = ["full"] }
futures = { version = "0.3.28", features = [] }
lazy_static = "1.4.0"
sqlx = { version = "0.7.3", features = ["postgres"]}
dotenv = "0.15.0"
tokio-postgres = {version = "0.7.10", features = ["with-uuid-0_8", "with-serde_json-1"] }
database = "0.5.0"
apps = "0.2.2"
emoji-logger = "0.1.0"
deadpool-postgres = "0.11.0"
postgres-openssl = "0.5.0"
openssl = "0.10.60"
config = "0.13.4"
uuid = { version = "0.8.2" , features = ["serde"]}
derive_more = { version = "0.99.17", features = [] }
regex = "1.7.1"
dirs = "5.0.1"
async-std = { version = "1.12.0", features = [] }
tera = "1.19.1"
calamine = "0.22.1"
```

`app.rust.env` : Contains DB Connection Information

```ini
DATABASE_URL=postgres://USER:PASS@HOST/DBNAME
PG.USER=USER
PG.PASSWORD=PASS
PG.HOST=HOST
PG.PORT=5432
PG.DBNAME=DBNAME
PG.POOL.MAX_SIZE=16
```

`src/main.rs`

```rust
// main.rs

use std::arch::aarch64::int8x8_t;
use std::collections::HashMap;
use std::env;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, http, ResponseError};
use actix_cors::Cors;
use actix_web::http::header::CONTENT_TYPE;

use actix_web::web::get;
use serde::{Deserialize, Serialize};
use tokio_postgres::{NoTls, Error, Row};
use deadpool_postgres::Client;
use tokio::runtime::Runtime;
use serde_json::json;
use regex::Regex;

use async_std::task;
use std::time::Duration;
use actix_web::web::Query;
use futures::{join, select, StreamExt};
use futures::future::FutureExt;
use futures::stream;
use futures::pin_mut;
use futures::try_join;
use async_std;
use deadpool_postgres::{Config, Pool};
use derive_more::{Display, From};
use dotenv::dotenv;
use tera::{Context, Tera};
use actix_web::{web::Data};

#[derive(Serialize, Deserialize, Debug)]
struct TRandom {
    random_num: i32,
    random_float: f64,
    md5: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Data1 {
    // Define your data structure
    random_num: i32,
    random_float: f64,
    md5: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PaginationParams {
    page: u32,
    page_size: u32,
}


#[derive(Serialize, Deserialize, Debug)]
struct QueryParams {
    // pagination: PaginationParams,
    // page: u32,
    // page_size: u32,
    start: u32,
    length: u32,
    draw: u32,
}

#[derive(Deserialize)]
struct FormData {
    // No specific fields defined
    // Allows capturing all fields dynamically
    #[serde(flatten)]
    fields: HashMap<String, String>,
}

#[derive(Display, From, Debug)]
enum CustomError {
    DatabaseError,
    InvalidData,
    QueryError,
    InvalidTable,
}
impl std::error::Error for CustomError {}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            CustomError::DatabaseError => HttpResponse::InternalServerError().finish(),
            CustomError::InvalidData => HttpResponse::BadRequest().finish(),
            CustomError::QueryError => HttpResponse::BadRequest().finish(),
            CustomError::InvalidTable => HttpResponse::BadRequest().finish(),
        }
    }
}


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// query_params: web::Json<QueryParams>

async fn make_db_pool() -> Pool {
    dotenv().ok();
    dotenv::from_filename("app.rust.env").ok();

    let mut cfg = Config::new();
    cfg.host = Option::from(env::var("PG.HOST").unwrap());
    cfg.user = Option::from(env::var("PG.USER").unwrap());
    cfg.password = Option::from(env::var("PG.PASSWORD").unwrap());
    cfg.dbname = Option::from(env::var("PG.DBNAME").unwrap());
    let pool: Pool = cfg.create_pool(None, tokio_postgres::NoTls).unwrap();
    pool
}


async fn get_count(table_name: &str, pool: Pool) -> Result<i64, Error> {
    let conn = pool.get().await.unwrap();
    let sql_query = format!("select count(*) from {}", table_name);
    let stmt = conn.prepare(sql_query.as_str()).await.unwrap();
    let row = conn.query_one(&stmt, &[]).await.unwrap();
    Ok(row.get(0))
}

// async fn query_data(web::Form(form): web::Form<QueryParams>) -> impl Responder {
// async fn query_data(form: web::Form<web::UrlEncoded<web::Payload>>) -> impl Responder {
#[post("/query")]
async fn query_data(form: web::Form<FormData>) -> impl Responder {

    // let form_data = &form.into_inner();

    let mut draw: u32 = 0;
    let mut length: u32 = 0;
    let mut start: u32 = 0;
    let mut search_string = "".to_string();
    let mut exact_search = "".to_string();

    let mut search_strings:Vec<String> = vec![];

    let mut sanitized_search_strings: Vec<String> = vec![];

    let mut search_string_for_reference = "".to_string();

    let mut search_type = "or".to_string();

    let mut sanitized_search_strings = vec![];
    let mut search_strings_with_leading_and_trailing_spaces_removed = vec![];

    let mut search_string_for_reference = "".to_string();

    // Access all form fields dynamically
    for (key, value) in &form.fields {
        println!("Field: {} : {}", key, value);

        if key == "length" {
            length = value.parse::<u32>().unwrap();
        }
        if key == "draw" {
            draw = value.parse::<u32>().unwrap() as u32;
        }
        if key == "start" {
            start = value.parse::<u32>().unwrap() as u32;
        }

        if key == "search[value]" {
            let search_string = value.to_string();
            search_string_for_reference = value.to_string();
            println!("search_string_for_reference: {}", search_string_for_reference);

            if !search_string.is_empty() {
                // you cannot have both AND ('+') and OR ('|') based search
                // it should be either '+' or '|'
                if search_string.contains("+") && search_string.contains("|") {
                    println!("error : cannot search, search_string contains both '+' (AND search) and '|' (OR search) !");
                    return HttpResponse::BadRequest().finish()
                }
            }

            if search_string.contains("+") { // AND search
                search_strings = split_string(search_string.as_str(), "+").await;
                search_type = "and".to_string();
            } else if search_string.contains("|") { // OR search
                search_strings = split_string(search_string.as_str(), "|").await;
                search_type = "or".to_string();
            } else { // default search, which means (search_string) does not contain either '+' or '|'
                search_type = "or".to_string();
                search_strings = Vec::from([search_string]);
            }

            // below sanitize_string(...) will ensure to remove
            // all characters except these : "_./-@,#:;"
            // so that way, DB does not get any characters that are not needed
            // let mut search_strings_with_leading_and_trailing_spaces_removed: Vec<String> = search_strings.iter().map(|s| remove_leading_and_trailing_spaces(s)).collect();

            let futures_1: Vec<_> = search_strings.iter().map(|s| async {
                remove_leading_and_trailing_spaces(s).await
            }).collect();
            search_strings_with_leading_and_trailing_spaces_removed = futures::future::join_all(futures_1).await;

            println!("@ length of search_strings_with_leading_and_trailing_spaces_removed >> {}", search_strings_with_leading_and_trailing_spaces_removed.len());

            // //////////////////////////

            // sanitized_search_strings = search_strings_with_leading_and_trailing_spaces_removed.iter().map(|s| sanitize_string(s)).collect();

            let futures_2: Vec<_> = search_strings_with_leading_and_trailing_spaces_removed.iter().map(|s| async {
                sanitize_string(s).await
            }).collect();
            sanitized_search_strings = futures::future::join_all(futures_2).await;

            println!("@ length of sanitized_search_strings >> {}", sanitized_search_strings.len());

            // //////////////////////////

            println!("sanitized_search_strings: {:#?}", sanitized_search_strings);

            println!("@ length of sanitized_search_strings >> {}", sanitized_search_strings.len());
        }

        if key == "exactsearch" {
            exact_search = value.to_string();
        }
    }

    let mut actual_db_table = "t_random".to_string();
    println!("actual_db_table: [ {} ]", actual_db_table.to_string());

    let mut default_query = "".to_string();

    /*
        get_inner_query(table_columns: Vec<String>, search_strings: Vec<String>, pattern_match: String, search_type: String) -> Result<String, CustomError>
    */

    let mut table_columns = vec![];
    table_columns.push("random_num".to_string());
    table_columns.push("random_float".to_string());
    table_columns.push("md5".to_string());

    let mut pattern_match = "".to_string();
    if exact_search == "true" {
        pattern_match = "exact".to_string();
    } else if exact_search == "false" {
        pattern_match = "like".to_string();
    } else {
        return HttpResponse::BadRequest().finish()
    }

    println!("pattern_match: [ {} ]", pattern_match.to_string());

    let mut inner_query = "".to_string();

    if search_string_for_reference.is_empty() {
        default_query = format!("SELECT * FROM {} LIMIT {} OFFSET {}", actual_db_table, length, start);
        println!("default_query (1) : [ {} ]", default_query.to_string());
    } else {
        println!("table_columns : {:#?} , sanitized_search_strings : {:#?} , pattern_match : {} , search_type : {}", table_columns, sanitized_search_strings, pattern_match, search_type);
        inner_query = get_inner_query(table_columns, sanitized_search_strings, pattern_match, search_type).await.unwrap();
        println!("inner_query (2) : {}", inner_query.to_string());

        default_query = format!("SELECT * FROM {} WHERE {} LIMIT {} OFFSET {}", actual_db_table, inner_query, length, start);
        println!("default_query (2) : {}", default_query.to_string());
    }

    let my_db_pool = get_db_pool_for_table("table1").await.unwrap();

    let client: deadpool_postgres::Client = my_db_pool.get().await.unwrap();

    let rows = client.query(default_query.as_str(), &[]).await.map_err(|_| HttpResponse::BadRequest().finish()).unwrap();

    let mut structs_1:Vec<Data1> = Vec::new();

    for row in rows {
        let random_num = row.get("random_num");
        let random_float = row.get("random_float");
        let md5 = row.get("md5");

        let my_struct = Data1 {
            random_num,
            random_float,
            md5,
        };
        structs_1.push(my_struct)
    };

    let records_total = get_count(actual_db_table.as_str(), my_db_pool).await.unwrap();
    println!("records_total : {}", records_total);

    let records_filtered = records_total;
    let response = json!({
        "data": structs_1,
        "draw": draw,
        "recordsFiltered": records_filtered,
        "recordsTotal": records_total,
    });
    HttpResponse::Ok().json(response)
}

async fn get_db_pool_for_table(source_table: &str) -> Result<Pool,CustomError> {
    println!("source_table : {}", source_table.to_string());
    return match source_table {
        "table1" => {
            Ok(make_db_pool().await)
        },
        _ => {
            return Err(CustomError::InvalidTable)
        }
    };
}

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", "Data Table");
    context.insert("message", "PS Table");

    let rendered = tera.render("index.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    dotenv::from_filename("app.rust.env").ok();

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let tera = Tera::new("templates/**/*").unwrap();

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(actix_web::web::Data::new(tera.clone()))
            .wrap(cors)
            .service(query_data)
            .route("/tables", web::get().to(index))
    })
    .bind("0.0.0.0:5050")?
    .run()
    .await
}

fn get_sanitized_string(text: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9-._@/,;:\s]+").unwrap();
    let result = re.replace_all(&text, "");
    result.to_string()
}

/* ************************************************************************************* */

async fn sanitize_string(input: &str) -> String {
    input.chars()
        .filter(|&c| c.is_ascii_alphanumeric() || "_./-@,#:;".contains(c))
        .collect()
}

async fn remove_leading_trailing_characters(input: &str) -> String {
    input.trim_start_matches(",").trim_end_matches(",").to_string()
}

async fn replace_multiple_characters(input: &str) -> String {
    let re = Regex::new(r",+").unwrap();
    re.replace_all(input, ",").to_string()
}

async fn split_string(input: &str, split_char: &str) -> Vec<String> {
    input.split(split_char).map(|s| s.to_string()).collect()
}

async fn remove_leading_and_trailing_spaces(my_str: &str) -> String {
    my_str.trim_start().trim_end().to_string()
}

/* ************************************************************************************* */

async fn get_inner_query(table_columns: Vec<String>, search_strings: Vec<String>, pattern_match: String, search_type: String) -> Result<String, CustomError> {
    let mut inner_query = "".to_string();

    if !(search_type == "and" || search_type == "or") {
        println!("error : search_type is neither 'and' nor 'or' !");
        return Err(CustomError::QueryError)
    }

    if !(pattern_match == "like" || pattern_match == "exact") {
        println!("error : pattern_match is neither 'like' nor 'exact' !");
        return Err(CustomError::QueryError)
    }

    if table_columns.len() == 0 {
        println!("error : table_columns length is ZERO !");
        return Err(CustomError::QueryError)
    }

    if search_strings.len() == 0 {
        println!("error : search_strings length is ZERO !");
        return Err(CustomError::QueryError)
    }

    if pattern_match.as_str() == "exact" { // exact string match
        // search all JSON fields for possible match
        // this can also be applied if there are other columns
        if search_type == "and" {
            inner_query = inner_query + " ( ";
            let mut search_string_counter = 1;
            for my_search_str in search_strings.to_owned() {
                inner_query = inner_query + " ( ";
                let mut column_counter = 1;
                // --------------------------------------
                for my_column in table_columns.to_owned() {
                    if column_counter == table_columns.len() as i32 {
                        inner_query = inner_query + format!(" lower({}::text) = lower('{}') ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    } else {
                        inner_query = inner_query + format!(" lower({}::text) = lower('{}') OR ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    }
                    column_counter += 1;
                }
                if search_string_counter == search_strings.len() as i32 {
                    inner_query = inner_query + " ) ";
                } else {
                    inner_query = inner_query + " ) AND ";
                }
                search_string_counter += 1;
                // --------------------------------------
            }
            inner_query = inner_query + " ) ";
        } else if search_type == "or" {
            inner_query = inner_query + " ( ";
            let mut search_string_counter = 1;
            for my_search_str in search_strings.to_owned() {
                inner_query = inner_query + " ( ";
                let mut column_counter = 1;
                // --------------------------------------
                for my_column in table_columns.to_owned() {
                    if column_counter == table_columns.len() as i32 {
                        inner_query = inner_query + format!(" lower({}::text) = lower('{}') ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    } else {
                        inner_query = inner_query + format!(" lower({}::text) = lower('{}') OR ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    }
                    column_counter += 1;
                }
                if search_string_counter == search_strings.len() as i32 {
                    inner_query = inner_query + " ) ";
                } else {
                    inner_query = inner_query + " ) OR ";
                }
                search_string_counter += 1;
                // --------------------------------------
            }
            inner_query = inner_query + " ) ";
        } else {
            return Err(CustomError::QueryError)
        }

    } else if pattern_match.as_str() == "like" { // pattern match
        // search all JSON fields for possible match
        // this can also be applied if there are other columns
        if search_type == "and" {
            inner_query = inner_query + " ( ";
            let mut search_string_counter = 1;
            for my_search_str in search_strings.to_owned() {
                inner_query = inner_query + " ( ";
                let mut column_counter = 1;
                // --------------------------------------
                for my_column in table_columns.to_owned() {
                    if column_counter == table_columns.len() as i32 {
                        inner_query = inner_query + format!(" lower({}::text) like lower('%{}%') ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    } else {
                        inner_query = inner_query + format!(" lower({}::text) like lower('%{}%') OR ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    }
                    column_counter += 1;
                }
                if search_string_counter == search_strings.len() as i32 {
                    inner_query = inner_query + " ) ";
                } else {
                    inner_query = inner_query + " ) AND ";
                }
                search_string_counter += 1;
                // --------------------------------------
            }
            inner_query = inner_query + " ) ";
        } else if search_type == "or" {
            inner_query = inner_query + " ( ";
            let mut search_string_counter = 1;
            for my_search_str in search_strings.to_owned() {
                inner_query = inner_query + " ( ";
                let mut column_counter = 1;
                // --------------------------------------
                for my_column in table_columns.to_owned() {
                    if column_counter == table_columns.len() as i32 {
                        inner_query = inner_query + format!(" lower({}::text) like lower('%{}%') ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    } else {
                        inner_query = inner_query + format!(" lower({}::text) like lower('%{}%') OR ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    }
                    column_counter += 1;
                }
                if search_string_counter == search_strings.len() as i32 {
                    inner_query = inner_query + " ) ";
                } else {
                    inner_query = inner_query + " ) OR ";
                }
                search_string_counter += 1;
                // --------------------------------------
            }
            inner_query = inner_query + " ) ";
        } else {
            return Err(CustomError::QueryError)
        }
    } else {
        return Err(CustomError::QueryError)
    }
    Ok(inner_query)
}
```

`tamplates/index.html`

```html
<!DOCTYPE html>
<html>
<head>
    <title>DataTables Pagination</title>

    <script src="https://code.jquery.com/jquery-3.6.0.min.js"></script>

    <script src="https://cdn.datatables.net/1.10.25/js/jquery.dataTables.min.js"></script>
    <script src="https://cdn.datatables.net/buttons/2.4.0/js/dataTables.buttons.min.js"></script>
    <script src="https://cdn.datatables.net/fixedcolumns/4.3.0/js/dataTables.fixedColumns.min.js"></script>
    <script src="https://cdn.datatables.net/responsive/2.5.0/js/dataTables.responsive.min.js"></script>


    <link rel="stylesheet" type="text/css" href="https://cdn.datatables.net/1.10.25/css/jquery.dataTables.css">
    <link rel="stylesheet" href="https://cdn.datatables.net/fixedcolumns/4.3.0/css/fixedColumns.dataTables.min.css">
    <link rel="stylesheet" href="https://cdn.datatables.net/responsive/2.5.0/css/responsive.dataTables.min.css">

    <link rel="stylesheet" href="https://www.w3schools.com/w3css/4/w3.css">
    <link rel="stylesheet" href="https://www.w3schools.com/lib/w3-colors-2021.css">
    <link rel="stylesheet" href="https://www.w3schools.com/lib/w3-colors-2020.css">
    <link rel="stylesheet" href="https://www.w3schools.com/lib/w3-colors-win8.css">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">

    <!-- --------------------------------- ADD ONS | start ----------------------------------------------- -->

    <script src=""></script>
    <link rel="stylesheet" href="">

    <script src="https://cdn.datatables.net/buttons/2.4.2/js/dataTables.buttons.min.js"></script>
    <script src="https://cdn.datatables.net/buttons/2.4.2/js/buttons.html5.min.js"></script>

    <link rel="stylesheet" href="https://cdn.datatables.net/buttons/2.4.2/css/buttons.dataTables.min.css">


    <!-- --------------------------------- ADD ONS | end ----------------------------------------------- -->


    <link rel="stylesheet" href="">

    <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Roboto">

    <style>
    .custom-font {
        font-family: Roboto, Helvetica,sans-serif !important;
    }

    .my-body-margin {
        margin-left: 5px;
        margin-right: 5px;
        margin-top: 5px;
        margin-bottom: 5px;
    }

    .my-body-padding {
        padding-left: 5px;
        padding-right: 5px;
        padding-top: 5px;
        padding-bottom: 5px;
    }

    </style>

    <script>
        var object_table = {};
    </script>

</head>
<body class="custom-font my-body-margin my-body-padding">

    <div id="id_table_content">

        <input id="my_checkbox" name="my_checkbox" class="w3-check" checked="checked" type="checkbox">&nbsp;&nbsp;<label>Exact Search</label>
        <br/>
        <br/>

        <table id="dataTable" class="display cell-border nowrap" style="width:100%">
            <thead>
                <tr>
                    <th>random_num</th>
                    <th>random_float</th>
                    <th>md5</th>
                </tr>
            </thead>
            <tbody>
                <!-- jQuery Data Table Body -->
            </tbody>
        </table>

    </div> <!-- id_table_content -->

</body>
</html>


<script>
window.addEventListener('DOMContentLoaded', (event) => {
    render_table();
});
</script>

<script>
function render_table() {

    var exact_search_bool = $("#my_checkbox").is(":checked");

    var exact_search = "";

    if (exact_search_bool === true) {
        exact_search = "yes";
    } else {
        exact_search = "no";
    }

    object_table = $('#dataTable').DataTable({
        "lengthMenu": [10, 50, 100, 500, 1000],
        "pageLength": 10,
        "processing": true,
        "serverSide": true,
        "deferRender": true,
        "searching": true,
        "scrollY": "400px",
        "sScrollX": '100%',
        "dom": "Bfrtip",
        "ajax": {
            "url": '/query',
            "type" : "POST",
            "data" : function(d) {
                d.exactsearch = $("#my_checkbox").is(":checked");
            }
        },
        "drawCallback": function(oSettings, json){
    
        },
        "columns" : [
            {"data": "random_num"},
            {"data": "random_float"},
            {"data": "md5"}
        ],
        "language" : {
            "processing": '<div class="lds-dual-ring"></div>'
        },
        'columnDefs': [
            {"targets": 0, "className": "dt-body-center"},
            {"targets": 1, "className": "dt-body-center"},
            {"targets": 2, "className": "dt-body-center"}
        ]
    }); // $('#dataTable').DataTable({


    $("#id_table_content").find('input[type="search"]').css("text-align", "center");
    $("#id_table_content").find('input[type="search"]').css("width", "500px");
    $("#id_table_content").find('input[type="search"]').css("background-color", "#cdffb1");

    $('#id_table_content').find('input[type="search"]').off();


    $('#id_table_content').find('input[type="search"]').on('keyup', function(e) {
        if(e.keyCode === 13) {
            console.log("--[searching for this.value]--");
            console.log(this.value);
            var stringToSearch = this.value;
            if(stringToSearch.includes("+") === true && stringToSearch.includes("|")) {
                alert("Your Search Cannot Include BOTH '+' and '|' (OR) Search Characters.");
                return false;
            }
            object_table.search( this.value ).draw();
        }
    });

} // function render_table() {
</script>
```