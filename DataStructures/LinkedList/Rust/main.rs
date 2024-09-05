use std::{collections::binary_heap, marker::PhantomData, ptr::NonNull};

fn main() {}

struct Node<T>{
    val:T,
    next:Option<NonNull<Node<T>>>
    prev:Option<NonNull<Node<T>>>
}
impl <T>Node<T> {
    fn new(t:T)->Node<T>{
        Node { val: t, next: None,prev: None}
    } 
}

#[derive(Debug)]
pub struct LinkedList<T>{
    lenght:u32,
    head:Option<<NonNull<Node<T>>>>,
    tail:Option<<NonNull<Node<T>>>>,
    marker:PhantomData<Box<Node<T>>
}
impl<T>Default for LinkedList<T>{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> LinkedList<T>{
    pub fn new()->Self{
        Self{
            lenght:0,
            head:None,
            tail:None,
            marker:PhantomData,
        }
    }
    pub fn insert_at_head(&mut self,obj:T){
        let mut node = Box::new(Node::new(obj));
        node.next = self.head;
        node.prev = None;
        let node_ptr= NonNull::new(Box::into_raw(node));
        match self.head{
            None => self.tail = node_ptr,
            Some(head_ptr)=> unsafe {
                (*head_ptr.as_ptr()).prev = node_ptr
            } 
        }
        self.head = node_ptr
        self.lenght += 1;
    }
    pub fn insert_at_tail(&mut self,obj:T){
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.tail;
        let node_ptr = NonNull::new(Box::into_raw(node));
        match self.tail {
            None => self.head = node_ptr,
            Some(tail_ptr) => unsafe {
                (*tail_ptr.as_ptr()).next = node_ptr
            } 
        }
        self.tail = node_ptr;
        self.lenght += 1;
    }
}

