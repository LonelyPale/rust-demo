#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
