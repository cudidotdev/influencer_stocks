use cosmwasm_std::Response;

pub fn has_attribute(response: &Response, key: &str, value: &str) -> bool {
    response
        .attributes
        .iter()
        .any(|attr| attr.key == key && attr.value == value)
}
