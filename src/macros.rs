
macro_rules! vk_try {
    ( $e:expr ) => {
        let result = $e;
        if result != VK_SUCCESS {
            return Err(From::from(result));
        }
    }
}
