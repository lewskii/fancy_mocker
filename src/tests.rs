#[cfg(test)]
mod starting_case_tests {
    #[test]
    fn should_start_uppercase_with_even_l_index() {
        assert_eq!(crate::start_uppercase("hello"), true)
    }
    #[test]
    fn should_start_lowercase_with_even_i_index() {
        assert_eq!(crate::start_uppercase("hei"), false)
    }
    #[test]
    fn should_start_lowercase_with_odd_l_index() {
        assert_eq!(crate::start_uppercase("ello"), false)
    }
    #[test]
    fn should_start_uppercase_with_odd_i_index() {
        assert_eq!(crate::start_uppercase("ei"), true)
    }
    #[test]
    fn should_start_lowercase_with_even_i_before_l() {
        assert_eq!(crate::start_uppercase("heil"), false)
    }
    #[test]
    fn should_start_uppercase_with_even_l_before_i() {
        assert_eq!(crate::start_uppercase("heli"), true)
    }
    #[test]
    fn should_start_lowercase_with_odd_i_before_l() {
        assert_eq!(crate::start_uppercase("eil"), true)
    }
    #[test]
    fn should_start_uppercase_with_odd_l_before_i() {
        assert_eq!(crate::start_uppercase("eli"), false)
    }
}
#[cfg(test)]
mod uppercase_checking_tests {
    #[test]
    fn should_return_true_for_lowercase_l() {
        assert_eq!(crate::should_be_uppercase('l'), Some(true))
    }
    #[test]
    fn should_return_true_for_uppercase_l() {
        assert_eq!(crate::should_be_uppercase('L'), Some(true))
    }
    #[test]
    fn should_return_false_for_lowercase_i() {
        assert_eq!(crate::should_be_uppercase('i'), Some(false))
    }
    #[test]
    fn should_return_false_for_uppercase_i() {
        assert_eq!(crate::should_be_uppercase('I'), Some(false))
    }
}
#[cfg(test)]
mod mock_tests {
    #[test]
    fn too_tired_to_name() {
        assert_eq!(crate::mock("Hello, world!"), String::from("HeLLo, WoRLd!"));
    }
}