pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct CrudJson;
impl CrudJson {
    pub fn read_json(&self, file_path: &str) {
        println!("{}", file_path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn json_test() {
        let crud_json = CrudJson;

        crud_json.read_json("test.json");
    }
}
