pub fn register<F: FnMut(String) + Send + 'static>(
    identifier: &str,
    scheme: &str,
    handler: F,
) -> Result<(), std::io::Error> {
    unimplemented!()
}

pub fn unregister(scheme: &str) -> Result<(), std::io::Error> {
    unimplemented!()
}
