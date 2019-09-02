pub trait Show {

    fn show(&self) -> String;
}

impl Show for String {
    fn show(&self) -> String {
        self.to_string()
    }
}

impl <T: Show> Show for [T] {
    fn show(&self) -> String {
        let mut temp = self.into_iter().map(|x| x.show()).fold("".to_string(), |mut acc, x| {acc.push_str(x.as_str()); acc.push_str(","); acc});
        if temp.is_empty() {temp} else { temp.pop(); temp }
    }
}
