/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	q1: RefCell<Queue<T>>,
	q2: RefCell<Queue<T>>
}
impl<T: Clone + Copy> myStack<T> {
    pub fn new() -> Self {
        Self {
			q1: RefCell::new(Queue::<T>::new()),
			q2: RefCell::new(Queue::<T>::new()),
        }
    }
    pub fn push(&mut self, elem: T) {
        self.q1.borrow_mut().enqueue(elem);
    }
    pub fn pop(&self) -> Result<T, &str> {
        if self.is_empty() {
            return Err("Stack is empty");
        }
        // use q2 as an aid queue for moving elements
        let q1 = &self.q1;
        let q2 = &self.q2;
        
        // move first n -1 elements to q2
        let q1_size = q1.borrow().size();
        for _i in 0..(q1_size - 1) {
            let mut queue = q1.borrow_mut();
            match queue.dequeue() {
                Ok(value) => q2.borrow_mut().enqueue(value),
                Err(_) => ()
            }
            
        }

        // move q2's elements back to q1
        let q2_size = q2.borrow().size();
        for _i in 0..q2_size {
            let mut queue = q2.borrow_mut();
            match queue.dequeue() {
                Ok(value) => q1.borrow_mut().enqueue(value),
                Err(_) => ()
            }
        }

        match q1.borrow_mut().dequeue() {
            Ok(value) => return Ok(value),
            Err(_) => return Err("Stack is empty")
        }
    }
    pub fn is_empty(&self) -> bool {
        self.q1.borrow().is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}