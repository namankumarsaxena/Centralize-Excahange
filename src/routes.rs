use actix_web::{HttpResponse, Responder, delete, get, post, web::Json};

use crate::{inputs::{CreateOrderInput, DeleteOrder}, outputs::{CreateOrderResponse, DeleteOrderResponse, Depth},
orderbook::{Orderbook}};

#[post("/order")]
pub async fn create_order(body: Json<CreateOrderInput>) -> impl Responder {
    let body = body.0;

    let price = body.price;
    let quantity = body.quantity;
    let user_id = body.user_id;
    let side = body.side;

    HttpResponse::Ok().json(CreateOrderResponse{
        order_id: user_id.to_string()
    })
}

#[delete("/order")]
pub async fn delete_order(Json(body): Json<DeleteOrder>, orderbook: Orderbook) -> impl Responder {
    let order_id = body.order_id;
    
    HttpResponse::Ok().json(DeleteOrderResponse{
        filled_qty: 0,
        average_price: 100
    })
}

#[get("/depth")]
pub async fn get_depth(orderbook: Orderbook) -> impl Responder {
 HttpResponse::Ok().json(Depth {
    bids: vec![],
    asks: vec![],
    lastUpdateId: String::from("Naman")
 })
    }