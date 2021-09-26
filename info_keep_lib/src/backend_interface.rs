pub trait BackendDatabase {
    type Db;
    type ValueHandler;

    fn open(path: &str)->Self::Db;

    fn insert<K,V>(&self,key: K, value: V);

    fn iter() -> dyn Iterator;
}

