use std::collections::HashMap;

pub fn init(address: String, amount: u32) -> HashMap<String, u32> {
    let mut hashMap = HashMap::new();

    hashMap.insert(address, amount);
    return hashMap
}
