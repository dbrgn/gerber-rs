pub extern crate gerber_types as types;

#[derive(Debug)]
pub struct Layer {
    file_attributes: Vec<types::FileAttribute>,
}

impl Layer {
    pub fn new() -> Self {
        Layer {
            file_attributes: vec![],
        }
    }

    pub fn set_file_attribute(&mut self, attribute: types::FileAttribute) {
        self.file_attributes.push(attribute);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
