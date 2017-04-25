
macro_rules! vk_try {
    ( $e:expr ) => {
        let result = $e;
        if result != VK_SUCCESS {
            return Err(From::from(result));
        }
    }
}

#[macro_export]
macro_rules! vk_make_version {
    ($major: expr, $minor: expr, $patch: expr) => {
        (($major as u32) << 22) | (($minor as u32) << 12) | $patch as u32
    }
}
