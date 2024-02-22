pub struct RequestData {
    method: String,
    path: String,
}

impl RequestData {
    pub fn new(method: String, path: String) -> Self {
        RequestData { method, path }
    }

    pub fn get_method(&self) -> String {
        self.method.clone()
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }
}
