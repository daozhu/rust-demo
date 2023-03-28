// 链表
//  有序的数据项集合能保证数据的相对位置，可以高效地索引。数组和链表都能做到将数
//  据有序地收集起来并保存在相对的位置，所以数组和链表都可用于实现有序数据类型，比如
//  Rust 默认实现的Vec 就是用的数组这种有序集合

// 链表中的每项都可抽象成一个节点Node，节点保存了数据项和下一项位置


// 节点连接用 Box 指针（大小确定）， 因为确定大小才能分配内存
type Link<T>  = Option<Box<Node<T>>>;

// 链表定义
pub struct List<T> {
    size: usize, // 链表节点数
    head: Link<T>, // 头结点
}
// 链表节点
pub struct Node<T> {
    elem: T, // 数据
    next: Link<T>, // 下一个节点链接
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
        }
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    // 新节点，加在头部
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }
    // take 会取出值，并留下空位
    pub fn pop(&mut self) -> Option<T>{
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.elem
        })
    }
    // peek 不会，只能是引用
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
    // 可变引用
    pub fn peek_mut(&mut self) -> Option<&mut &T>{
        self.head.as_mut().map(|node| &mut node.elem)
    }
}