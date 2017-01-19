extern crate trusty_backend as backend;
extern crate trusty_frontend as frontend;
extern crate trusty_utils as utils;

pub struct Editor<T: frontend::FrontEnd> {
    backend: backend::BackEnd,
    frontend: T,
}

impl<T> Editor<T> where T: frontend::FrontEnd {
    pub fn new<K>(backend: backend::BackEnd, frontend: K) -> Editor<K> 
                  where K: frontend::FrontEnd {
        Editor { backend: backend, frontend: frontend }
    }
    // TODO
    pub fn start(&self) where T: frontend::FrontEnd {
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
