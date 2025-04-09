#[cfg(test)]
mod tests{
    use crate::CircularBuffer::CircularBuffer;
    #[test]
    fn test_insert_size(){
        let mut buffer = CircularBuffer::new(5);
        buffer.write(1);
        assert_eq!(buffer.size(), 1);
    }

    #[test]
    fn test_insert_check() {
        let mut buffer = CircularBuffer::new(5);
        buffer.write(1);
        assert_eq!(buffer.read(), Some(1));
    }

    #[test]
    fn test_insert_n() {
        let mut buffer = CircularBuffer::new(5);
        for i in 0..5 {
            buffer.write(i);
        }
        for i in 0..5 {
            assert_eq!(buffer.read(), Some(i));
        }
    }

    #[test]
    fn test_make_contiguous() {
        let mut buffer = CircularBuffer::new(5);
        for i in 0..5 {
            buffer.write(i);
        }
        buffer.read();
        buffer.make_contiguous();
        assert_eq!(buffer.size(), 4);
    }

    #[test]
    fn test_overwrite() {
        let mut buffer = CircularBuffer::new(2);
        buffer.write(1);
        buffer.overwrite(2);
        assert_eq!(buffer.read(), Some(1));
        assert_eq!(buffer.read(), Some(2));
        assert_eq!(buffer.size(), 0);
    }
}
