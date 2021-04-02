mod multiselect;
mod select;

#[cfg(test)]
mod tests {
    use super::multiselect;
    use super::select;

    #[test]
    fn test_multiselect() {
        let mut options = vec!["Line 1", "This is line 2", "Some other line"];
        let ret = multiselect::ask_multiselect("Which line ?", &mut options).unwrap();
        assert_eq!(ret, &vec!["Line 1", "Some other line"]);
    }

    #[test]
    fn test_select() {
        let mut options = vec!["Line 1", "This is line 2", "Some other line"];
        let ret = select::ask_select("Which line ?", &mut options).unwrap();
        assert_eq!(ret, "Some other line");
    }
}
