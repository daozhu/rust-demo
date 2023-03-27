// 定义一个队列
#[derive(Debug)]
pub struct Queue<T> {
    cap: usize, // 容量
    data: Vec<T>, // 数据
}

impl<T> Queue<T> {
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
    // 判断是否有剩余空间，有则将数据加入空间
    pub fn enqueue(&mut self, item: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("no space available".to_string());
        }
        self.data.insert(0, item);

        Ok(())
    }
    // 数据出队
    pub fn dequeue(&mut self) -> Option<T> {
        if Self::is_empty(&self) {
            return None;
        }
        self.data.pop()
    }
}

pub fn test_queue(){
    let mut q1:Queue<usize> = Queue::new(3);

    let _r1 = q1.enqueue(1);
    let _r2 = q1.enqueue(2);
    let _r3 = q1.enqueue(3);

    if let Err(error) = q1.enqueue(4) {
        println!("enqueue error : {}", error);
    }
    if let Some(item) = q1.dequeue() {
        println!("dequeue : {}", item);
    }

    println!("size : {} , empty : {}", q1.size(), q1.is_empty());
    println!("content : {:?}", q1);
}

pub fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    let mut queue = Queue::new(names.len());
    for name in names {
        let _rm = queue.enqueue(name);
    }

    while queue.size() > 1 {
        for _ in 0..num {
            let name = queue.dequeue().unwrap();
            let _rm = queue.enqueue(name);
        }
        let _rm = queue.dequeue();
    }

    queue.dequeue().unwrap()
}

pub fn test_hot_potato(){
    let names = vec!["a", "b", "c", "d", "e", "f"];
    println!("{}", hot_potato(names, 7));
}