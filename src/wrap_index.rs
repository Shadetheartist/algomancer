
pub fn wrap_index(len: usize, idx: i32) -> Option<usize> {
    if len == 0 {
        return Some(len)
    }

    if idx == 0 {
        return Some(len)
    }

    // on the off-chance we can't actually compute this
    if len > i32::MAX as usize {
        return None
    }

    let i_len = len as i32;

    if idx >= 0 {
        return Some((idx % i_len) as usize)
    } else {
        let abs_idx = idx.abs() - 1;
        let e = abs_idx % i_len;
        let f = (i_len - e) - 1;
        Some(f as usize)
    }
}

#[cfg(test)]
mod tests {
    use std::usize;
    use crate::wrap_index::wrap_index;

    #[test]
    fn test_wrap_index(){
        assert_eq!(wrap_index(usize::MAX, 333).unwrap(), 333);
        assert_eq!(wrap_index(5, 5).unwrap(), 0);
        assert_eq!(wrap_index(5, 6).unwrap(), 1);
        assert_eq!(wrap_index(5, -1).unwrap(), 4);
        assert_eq!(wrap_index(5, -5).unwrap(), 0);
        assert_eq!(wrap_index(10, -5).unwrap(), 5);
        assert_eq!(wrap_index(0, -1).unwrap(), 0);
        assert_eq!(wrap_index(333, 0).unwrap(), 333);
    }
}
