pub fn shorten_address(address: String) -> String {
    format!("{}...{}", &address[0..5], &address[30..address.len()])
}
