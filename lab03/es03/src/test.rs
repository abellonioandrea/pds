#[cfg(test)]
mod tests{
    use crate::CircularBuffer::CircularBuffer;
    #[test]
    fn test_insert_size(){
        let mut buffer = CircularBuffer::new(5);
        buffer.write(1);
        assert_eq!(buffer.size(), 1);
    }
}
