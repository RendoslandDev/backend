
use actix_web::{get, web, HttpResponse, Responder, post, put, delete};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    id: String,
    name: String,
    price: f64,
    imageurl:String,
    description: String,
    link:String,
    category:String,
    total_products: i64,
    bought_products: i64,
}
#[derive(Debug , Deserialize)]
pub struct NewProduct{
    name: String,
    price: f64,
    imageurl:String,
    description: String,
    link:String,
    category:String,
}


pub struct ProductStore {
    products: Mutex<Vec<Product>>,
}

impl ProductStore {
    pub fn new() -> Self {
        let products = vec![
            Product {
                id: "1".to_string(),
                name: "Bueno Mattress".to_string(),
                price: 1399.0,
                imageurl: "https://ashfoam.com/wp-content/uploads/2017/12/Bueno-300x300.jpg".to_string(),
                description: "Energy efficient refrigerator with power rating of 3 stars".to_string(),
                link:"#".to_string(),
                category:"Cloud Mattress".to_string(),
                total_products: 24,
                bought_products: 2,
            },
            Product {
                id: "2".to_string(),
                    name: "Raven Bed".to_string(),
                    price: 2223.,
                    imageurl:"https://ashfoam.com/wp-content/uploads/2024/12/1-300x300.png".to_string(),
                    description: "Comfortable and durable".to_string(),
                    link: '#'.to_string(),
                    category: "mattress couche".to_string(),
                    total_products: 24,
                    bought_products: 2,
            },
            Product {
                id: "3".to_string(),
                    name: "Adinkra Couch®".to_string(),
                    imageurl: "https://ashfoam.com/wp-content/uploads/2018/01/Adinkra-1-300x300.jpg".to_string(),
                    price: 1829.9,
                    description: "Comfortable and durable".to_string(),
                    link: '#'.to_string(),
                 category:"Cloud Bed".to_string(),
                    total_products: 24,
                    bought_products: 2,
            },
            Product {
                id: "4".to_string(),
                    name: "Como Dresser(with stool)® ".to_string(),
                    imageurl: "https://ashfoam.com/wp-content/uploads/2023/04/Como-Dresser-Colour-min-300x300.jpg".to_string(),
                    price: 1299.9,
                    description: "Comfortable and durable".to_string(),
                    link: '#'.to_string(),
                    category: "lobe".to_string(),
                    total_products: 24,
                    bought_products: 2,
            },
            Product {
                id: "5".to_string(),
                    name: "Como Grand TV® ".to_string(),
                    imageurl: "https://ashfoam.com/wp-content/uploads/2023/08/grand-tv-unit-300x300.jpg".to_string(),
                    price: 1129.9,
                    description: "Comfortable and durable".to_string(),
                    link: '#'.to_string(),
                    category:"Cloud Mattress".to_string(),
                    total_products: 24,
                    bought_products: 2,
            },
            Product {
                id: "6".to_string(),
                    name: "Kiwi Center Table And Stools®".to_string(),
                    imageurl: "https://ashfoam.com/wp-content/uploads/2023/07/362250846_683237893844011_5938099163715413427_n-300x300.jpg".to_string(),
                    price: 1819.24,
                    description: "Comfortable and durable".to_string(),
                    link: '#'.to_string(),
                    category: "Cloud Couche".to_string(),
                    total_products: 24,
                    bought_products: 2,
            },
            Product {
                id: "7".to_string(),
                    name: "Sealy Posturepedic® High Plush Mattress".to_string(),
                    imageurl:"https://ashfoam.com/wp-content/uploads/2022/07/3-300x300.jpg".to_string(),
                    price: 3828.9,
                    description: "Comfortable and durable".to_string(),
                    link: '#'.to_string(),
                    category: "Cloud Mattress".to_string(),
                    total_products: 24,
                    bought_products: 2,

            },
            Product {
                id: "8".to_string(),
                    name: "Shoe rack with seat".to_string(),
                    imageurl:"https://ashfoam.com/wp-content/uploads/2022/01/OTttoman-300x300.jpg".to_string(),
                    price: 2214.0,
                    description: "Comfortable and durable".to_string(),
                    link: '#'.to_string(),
                    category:"Cloud Bed".to_string(),
                    total_products: 24,
                    bought_products: 2,
            },
            Product {
                id: "9".to_string(),
                    name: "Ottoman Storage".to_string(),
                    imageurl:"https://ashfoam.com/wp-content/uploads/2025/01/Untitled-1-300x300.png".to_string(),
                    price: 2962.90,
                    description: "Comfortable and durable".to_string(),
                    link: '#'.to_string(),
                    category: "Cloud Mattress".to_string(),
                    total_products: 24,
                    bought_products: 2,
            },
            Product {
                id: "10".to_string(),
                    name: "Sealy Posturepedic® High Plush Mattress".to_string(),
                    imageurl: "https://ashfoam.com/wp-content/uploads/2023/08/tano-2-300x300.jpg".to_string(),
                    price: 1320.4,
                    description: "Comfortable and durable".to_string(),
                    link: '#'.to_string(),
                    category: "Cloud Bed".to_string(),
                    total_products: 24,
                    bought_products: 2,
            },
            Product {
                id: "11".to_string(),
                    name: "Sealy Posturepedic® High Plush Mattress".to_string(),
                    imageurl: "../.././Arrival/download (5).jpeg".to_string(),
                    price: 2166.0,
                    description: "the reason we broke up".to_string(),
                    link: '#'.to_string(),
                    category:"Cloud Mattress".to_string(),
            total_products: 24,
            bought_products: 2,
            },
            Product {
                id: "12".to_string(),
                    name: "Sealy Posturepedic® High Plush Mattress".to_string(),
                    imageurl: "../.././Arrival/download (4).jpeg".to_string(),
                    price: 2616.0,
                    description: "the reason we broke up".to_string(),
                    link: '#'.to_string(),
                    category: "Cloud Bed".to_string(),
                    total_products: 24,
            bought_products: 2,
            },
            Product {
                id: "13".to_string(),
                    name: "Sealy Posturepedic® High Plush Mattress".to_string(),
                    imageurl: "../.././Arrival/download (6).jpeg".to_string(),
                    price: 2121.0,
                    description: "the reason we broke up".to_string(),
                    link: '#'.to_string(),
                    category:"Cloud Bed".to_string(),
                    total_products: 24,
            bought_products: 2,
            },
            // Add more sample products...
        ];

        ProductStore {
            products: Mutex::new(products),
        }
    }
}



#[get("/product/limit/{limit}")]
pub async fn get_products_with_limit(
    data: web::Data<ProductStore>,
    limit: web::Path<usize>,
) -> impl Responder {
    let products = data.products.lock().unwrap();
    let limit = limit.into_inner();
    let limited_products = products.iter().take(limit).cloned().collect::<Vec<_>>();

    HttpResponse::Ok().json(json!({
        "products": limited_products
    }))
}



#[get("/product/{id}")]
pub async fn get_product_by_id(
    data: web::Data<ProductStore>,
    id: web::Path<String>,
) -> impl Responder{
   let products = data.products.lock().unwrap();
   let product = products.iter().find(|p| p.id == *id);

   match  product  {
    Some(p) => HttpResponse::Ok().json(json!({
        "success": true,
        "product": p
    })),
    None => HttpResponse::NotFound().json(json!({
        "success": false,
        "message": "Product not found"
    })),
   }
}


#[get("/product")]
pub async fn get_all_products(
    data: web::Data<ProductStore>,
) -> impl Responder{
    let products = data.products.lock().unwrap();

    HttpResponse::Ok().json(json!({
        "success":true,
        "products":products.clone()
    }))
}


#[get("/product/category/{category_id}")]
pub async fn get_products_by_category(
    data: web::Data<ProductStore>,
    category_id: web::Path<String>,
) -> impl Responder {
    let products = data.products.lock().unwrap();
    let category_id = category_id.into_inner();
    let category_products = products
        .iter()
        .filter(|p| p.category == category_id)
        .cloned()
        .collect::<Vec<_>>();

    HttpResponse::Ok().json(json!({
        "success":true,
        "products": category_products
    }))
}

//creating , updating , read, deleting products..!

#[post("/product")]
pub async fn create_product(
    data: web::Data<ProductStore>,
    new_product:  web::Json<NewProduct>,
) -> impl Responder {
    let mut products = data.products.lock().unwrap();


    let product = Product{
        id: uuid::Uuid::new_v4().to_string(),
        name: new_product.name.clone(),
        price: new_product.price,
        imageurl: new_product.imageurl.clone(),
        description: new_product.description.clone(),
        link: new_product.link.clone(),
        category:new_product.category.clone(),
        total_products: 0,
        bought_products: 0,
    };
    products.push(product.clone());
    HttpResponse::Created().json(json!({
        "success":true,
        "product":product
    }))
}



#[put("/product/{id}")]
pub async fn update_product(
    data: web::Data<ProductStore>,
    id: web::Path<String>,
    updated_product: web::Json<NewProduct>,
) -> impl Responder {
 let mut products = data.products.lock().unwrap();

 if let  Some(index) = products.iter().position(|p| p.id == *id){
      let current_product = &products[index];
    let product = Product {
      id: id.into_inner(),
            name: updated_product.name.clone(),
            price: updated_product.price,
            imageurl: updated_product.imageurl.clone(),
            description: updated_product.description.clone(),
            link: updated_product.link.clone(),
            category: updated_product.category.clone(),
            total_products: current_product.total_products,
            bought_products: current_product.bought_products,
    };

    products[index] = product.clone();
     HttpResponse::Ok().json(json!({
        "success":true,
        "meassage":product,
     }))
 } else{
    HttpResponse::NotFound().json(json!({
        "success":false,
        "message":"Product Not Found"
    }))
 }
}


#[delete("/product/{id}")]
pub async fn delete_product(
    data: web::Data<ProductStore>,
    id: web::Path<String>,
) -> impl Responder {
    let mut products = data.products.lock().unwrap();
    let initial_length = products.len();
    products.retain(|p| p.id != *id);

    if products.len() < initial_length{
 HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Product deleted successfully"
 }))
    }else{
        HttpResponse::NotFound().json(json!({
            "success": true,
            "message": "Product deleted successfully"
 }))
    }
}


#[put("/product/{id}/buy")]
pub async fn buy_product(
    data: web::Data<ProductStore>,
    id: web::Path<String>,
    quantity: web::Json<i64>,
) -> impl Responder {
    let mut products = data.products.lock().unwrap();

    if let Some(index) = products.iter().position(|p| p.id == *id) {
        products[index].bought_products += quantity.into_inner();
        HttpResponse::Ok().json(json!({
            "success": true,
            "product": products[index].clone()
        }))
    } else {
        HttpResponse::NotFound().json(json!({
            "success": false,
            "message": "Product not found"
        }))
    }
}
