#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue: Queue<i32> = Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None); // Fila vazia
    }

    #[test]
    fn test_peek() {
        let mut queue: Queue<i32> = Queue::new();

        queue.enqueue(10);
        queue.enqueue(20);

        assert_eq!(queue.peek(), Some(&10));

        queue.dequeue();
        assert_eq!(queue.peek(), Some(&20));

        queue.dequeue();
        assert_eq!(queue.peek(), None); // Fila vazia
    }

    #[test]
    fn test_len() {
        let mut queue: Queue<i32> = Queue::new();

        assert_eq!(queue.len(), 0);

        queue.enqueue(1);
        assert_eq!(queue.len(), 1);

        queue.enqueue(2);
        assert_eq!(queue.len(), 2);

        queue.dequeue();
        assert_eq!(queue.len(), 1);

        queue.dequeue();
        assert_eq!(queue.len(), 0);
    }
}