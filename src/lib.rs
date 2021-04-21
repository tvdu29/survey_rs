mod multiselect;
mod select;
mod input;

#[cfg(test)]
mod tests {
    use super::multiselect;
    use super::select;
    use super::input;

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

    #[test]
    fn test_input() {
        let ret = input::ask_input("What do you want to say ?").unwrap();
        assert_eq!(ret, "Some other line");
    }
}
