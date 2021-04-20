pub struct SeqList<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> SeqList<T, N> {
    pub fn retain<F>(&mut self, mut _f: F)
    where
        F: FnMut(&T) -> bool,
    {
    }
    /// 删除所有重复元素
    pub fn dedup(&mut self) {}
}

#[cfg(test)]
mod test {
    #[test]
    fn dedup_sorted() {
        /*
         * 时间上界 O(n)
         * high 指向第一个不同的元素，然后将其向前移动 space 个单位。
         */
        let mut arr1 = [1, 2, 2, 2, 3, 3, 4, 5, 6, 6, 7, 8, 8, 9, 10, 11, 11];
        let mut arr2 = arr1;
        fn dedup_sorted1(arr: &mut [i32]) {
            let (mut low, mut space) = (0, 0);
            let mut high;
            while low < arr.len() {
                high = low + 1;
                while high < arr.len() && arr[low] == arr[high] {
                    high += 1;
                }
                space += high - low - 1;
                if high == arr.len() {
                    arr[high - 1 - space] = arr[high - 1];
                } else {
                    arr[high - space] = arr[high];
                }
                low = high;
            }
            println!("{:?}, len:{}", arr, arr.len() - space);
        }
        dedup_sorted1(&mut arr1);

        fn dedup_sorted2(arr: &mut [i32]) {
            let mut i = 0;
            let mut j = 1;
            while j < arr.len() {
                if arr[i] != arr[j] {
                    i += 1;
                    arr[i] = arr[j]
                }
                j += 1;
            }
            println!("{:?}, len:{}", arr, i + 1);
        }
        dedup_sorted2(&mut arr2);
    }
    #[test]
    fn swap() {
        /*
         * 将数组的前 m 个元素和后 len-m 个元素交换位置。
         * 也可以先整体逆置，在各分部逆置（推荐）。
         */
        let mut arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let mut m_len = 9;
        let mut n_len = arr.len() - m_len;
        let mut m_idx = 0;
        let mut n_idx = m_len;
        loop {
            let min = if m_len > n_len { n_len } else { m_len };
            let mut i = 0;
            while i < min {
                let tmp = arr[m_idx + i];
                arr[m_idx + i] = arr[n_idx + i];
                arr[n_idx + i] = tmp;
                i += 1;
            }
            if m_len < n_len {
                n_len = n_len - m_len;
                m_idx += m_len;
                n_idx += m_len;
            } else if m_len > n_len {
                m_idx += n_len;
                m_len -= n_len;
            } else {
                break;
            }
        }
        println!("{:?}", arr);
    }
}
