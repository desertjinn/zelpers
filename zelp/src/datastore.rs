pub struct DataStore {
    location: String,
    file: String
}

impl DataStore {
    pub fn save<T>(&self, item: &T) {
        // todo: store to location
    }
}