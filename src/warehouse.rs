// 引当依頼
#[derive(Debug,PartialEq)]
pub struct OrderRequest {
    pub product_name: String,
    pub stock: i32
}
// 引当結果
#[derive(Debug,PartialEq)]
pub struct OrderResult {
    pub reserved: Vec<OrderReserved>,
    pub not_reserved: Vec<OrderNotReserved>,
}
// 引当結果(引当OK)
#[derive(Debug,PartialEq)]
pub struct OrderReserved {
    pub warehouse: String,
    pub product_name: String,
    pub stock: i32
}
// 引当結果(引当NG)
#[derive(Debug,PartialEq)]
pub struct OrderNotReserved {
    pub product_name: String,
    pub stock: i32
}
// 在庫情報
#[derive(Debug,PartialEq)]
pub struct InventoryItem {
    pub product_name: String,
    pub stock: i32
}
// 倉庫
#[derive(Debug,PartialEq)]
pub struct Warehouse<'a> {
    pub name: String,
    pub inventories: Vec<InventoryItem>,
    pub linked_warehouse: Option<&'a Warehouse<'a>>
}
impl Warehouse<'_> {
    pub fn reserve(&self, requests: &Vec<OrderRequest>) -> OrderResult {
        let mut order_result = OrderResult {
            reserved: Vec::new(),
            not_reserved: Vec::new(),
        };
        let mut requests_to_linked_warehouse = Vec::new();
        requests.iter().for_each(|request| {
            let (reserved, not_reserved) = self.reserve_item(&request);
            // 引当できた場合は引当結果(OK)として処理
            if let Some(r) = reserved {
                order_result.reserved.push(r);
            }
            // 引当できなかった場合はリンク先倉庫への注文依頼、または引当結果(NG)を生成
            if let Some(not_r) = not_reserved {
                match &self.linked_warehouse {
                    Some(_) => {
                        // リンク先倉庫が指定されている場合、注文依頼を作成
                        requests_to_linked_warehouse.push(OrderRequest{
                            product_name: String::from(&request.product_name),
                            stock: not_r.stock
                        })
                    },
                    None => {
                        // リンク先倉庫が指定されていない場合、引当結果(NG)として処理
                        order_result.not_reserved.push(not_r);
                    }
                }
            }
        });
        
        // リンク先倉庫への引当依頼
        if let Some(w) = &self.linked_warehouse {
            // リンク先倉庫が指定されている場合はリンク先倉庫に引当依頼
            let mut linked_order_result = w.reserve(&requests_to_linked_warehouse);
            // リンク先倉庫での引当結果をマージ
            order_result.reserved.append(&mut linked_order_result.reserved);
            order_result.not_reserved.append(&mut linked_order_result.not_reserved);
        }
        return order_result
    }

    // 1明細単位の在庫引当を実施
    // (本来はここがDBアクセスになり、行単位のトランザクションなので、このファンクションがトランザクションスコープ)
    fn reserve_item(&self, request: &OrderRequest) -> (Option<OrderReserved>, Option<OrderNotReserved>) {
        let reserved;
        let not_reserved;
        match &self.inventories.iter().find(|item| item.product_name == request.product_name) {
            Some(inventry_item) => {
                // 依頼された商品を扱っている場合は在庫数量を確認
                if &inventry_item.stock >= &request.stock {
                    // 在庫数が依頼数以上であれば、依頼された通りの数量を引当結果として返す
                    reserved = Some(OrderReserved{
                        warehouse: String::from(&self.name),
                        product_name: String::from(&request.product_name),
                        stock: request.stock
                    });
                    not_reserved = None;
                } else {
                    // 在庫数が依頼数に不足しているのであれば、在庫分全てを引き当てる
                    reserved = Some(OrderReserved{
                        warehouse: String::from(&self.name),
                        product_name: String::from(&request.product_name),
                        stock: inventry_item.stock
                    });
                    // 残りは不足分として返す
                    not_reserved = Some(OrderNotReserved{
                        product_name: String::from(&request.product_name),
                        stock: request.stock - inventry_item.stock
                    });
                }
            },
            None => {
                // 依頼された商品を扱っていない場合は依頼された通りの数量を不足分として返す
                reserved = None;
                not_reserved = Some(OrderNotReserved{
                    product_name: String::from(&request.product_name),
                    stock: request.stock
                });
            }
        }
        return (reserved, not_reserved)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    // 1明細引当依頼し、在庫が十分なため全て引当できる
    fn reserve_single_success() {
        let w1 = Warehouse {
			name: String::from("倉庫1"),
			inventories: vec![
				InventoryItem { product_name: String::from("商品A"), stock: 100 },
				InventoryItem { product_name: String::from("商品B"), stock: 100 },
				InventoryItem { product_name: String::from("商品C"), stock: 100 }
			],
			linked_warehouse: None
        };
        let result = w1.reserve(&vec![
            OrderRequest { product_name: String::from("商品A"), stock: 30 }
        ]);

        assert_eq!(result.reserved.len(), 1);
        assert_eq!(result.not_reserved.len(), 0);
        assert_eq!(result.reserved[0], OrderReserved{ warehouse: String::from("倉庫1"), product_name: String::from("商品A"), stock: 30 });
        
        assert_eq!(w1.inventories[0], InventoryItem { product_name: String::from("商品A"), stock: 100 }) // Warehouseはmutableなので引当後もstockは変化しない
    }

    #[test]
    // 2明細引当依頼し、1明細は在庫が不十分なため、一部引当できたが残りは引当できない
    // さらに1明細は商品自体を扱っていないため全て引当できない
    fn reserve_success_and_failure() {
        let w1 = Warehouse {
			name: String::from("倉庫1"),
			inventories: vec![
				InventoryItem { product_name: String::from("商品A"), stock: 100 },
				InventoryItem { product_name: String::from("商品B"), stock: 100 },
				InventoryItem { product_name: String::from("商品C"), stock: 100 }
			],
			linked_warehouse: None
        };
        let result = w1.reserve(&vec![
            OrderRequest { product_name: String::from("商品A"), stock: 150 },
            OrderRequest { product_name: String::from("商品X"), stock: 999 }
        ]);

        assert_eq!(result.reserved.len(), 1);
        assert_eq!(result.not_reserved.len(), 2);
        assert_eq!(result.reserved[0], OrderReserved{warehouse: String::from("倉庫1"), product_name: String::from("商品A"), stock: 100 });
        assert_eq!(result.not_reserved[0], OrderNotReserved{ product_name: String::from("商品A"), stock: 50 });
        assert_eq!(result.not_reserved[1], OrderNotReserved{ product_name: String::from("商品X"), stock: 999 });
    }

    #[test]
    // リンク先を3レベルまで辿る
	fn reserve_three_linked_warehouses() {
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

        assert_eq!(result.reserved.len(), 6);
        assert_eq!(result.not_reserved.len(), 0);
        assert_eq!(result.reserved[0], OrderReserved{ warehouse: String::from("倉庫3"), product_name: String::from("商品A"), stock: 50 });
        assert_eq!(result.reserved[1], OrderReserved{ warehouse: String::from("倉庫3"), product_name: String::from("商品B"), stock: 80 });
        assert_eq!(result.reserved[2], OrderReserved{ warehouse: String::from("倉庫3"), product_name: String::from("商品C"), stock: 60 });
        assert_eq!(result.reserved[3], OrderReserved{ warehouse: String::from("倉庫2"), product_name: String::from("商品B"), stock: 20 });
        assert_eq!(result.reserved[4], OrderReserved{ warehouse: String::from("倉庫2"), product_name: String::from("商品C"), stock: 20 });
        assert_eq!(result.reserved[5], OrderReserved{ warehouse: String::from("倉庫1"), product_name: String::from("商品C"), stock: 20 });
	}
}