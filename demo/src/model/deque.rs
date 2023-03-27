// 双端队列
#[derive(Debug)]
pub struct Deque<T> {
    cap: usize, // 容量
    data: Vec<T>, // 数据
}

impl<T> Deque<T> {
    pub fn new(cap: usize) -> Self {
        Self {
            cap,
            data: Vec::with_capacity(cap),
        }
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        Self::size(&self) == 0
    }
    pub fn add_near(&mut self, item: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("no space available".to_string());
        }
        self.data.insert(0, item);

        Ok(())
    }
    pub fn add_front(&mut self, item: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("no space available".to_string());
        }
        self.data.push(item);

        Ok(())
    }
    pub fn remove_front(&mut self) -> Option<T> {
        if Self::is_empty(&self) {
            return None;
        }
        self.data.pop()
    }
    pub fn remove_near(&mut self) -> Option<T> {
        if Self::size(&self) == 0 {
            return None;
        }

        Some(self.data.remove(0))
    }
}

// 回文
pub fn pal_checker(s: &str) -> bool{
    let mut deque = Deque::new(s.len());
    for c in s.chars() {
        let _rm = deque.add_near(c);
    }
    let mut is_pal = true;

    while deque.size() > 1 && is_pal {
        let head = deque.remove_front();
        let tail = deque.remove_near();
        if head != tail {
            is_pal = false;
        }
    }

    is_pal
}

pub fn test_pal_checker(){
    assert_eq!(pal_checker("abba"), true);
}
