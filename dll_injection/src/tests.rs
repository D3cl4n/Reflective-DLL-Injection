#[cfg(test)]

mod tests
{
    use crate::process::open_process;

    #[test]
    fn open_process_test()
    {
        unsafe
        {
            assert_ne!(open_process(6252), None)
        }
    }
}