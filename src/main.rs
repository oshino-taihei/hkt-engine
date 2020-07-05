mod warehouse;
use warehouse::*;

fn main() {
    // 倉庫1
    let w1 = Warehouse {
        name: String::from("倉庫1"),
        inventories: vec![
            InventoryItem { product_name: String::from("商品A"), stock: 100 },
            InventoryItem { product_name: String::from("商品B"), stock: 100 },
            InventoryItem { product_name: String::from("商品C"), stock: 100 }
        ],
        linked_warehouse: None
    };
    // 倉庫2
    let w2 = Warehouse {
        name: String::from("倉庫2"),
        inventories: vec![
            InventoryItem { product_name: String::from("商品A"), stock: 100 },
            InventoryItem { product_name: String::from("商品B"), stock: 20 },
            InventoryItem { product_name: String::from("商品C"), stock: 20 }
        ],
        linked_warehouse: Some(&w1)
    };
    // 倉庫3
    let w3 = Warehouse {
        name: String::from("倉庫3"),
        inventories: vec![
            InventoryItem { product_name: String::from("商品A"), stock: 200 },
            InventoryItem { product_name: String::from("商品B"), stock: 80 },
            InventoryItem { product_name: String::from("商品C"), stock: 60 }
        ],
        linked_warehouse: Some(&w2)
    };

    let result = w3.reserve(&vec![
        OrderRequest { product_name: String::from("商品A"), stock: 50 },
        OrderRequest { product_name: String::from("商品B"), stock: 100 },
        OrderRequest { product_name: String::from("商品C"), stock: 100 }
    ]);
    result.reserved.iter().for_each(|r| println!("{:?}", r));
}
