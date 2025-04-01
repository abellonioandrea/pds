pub struct CircularBuffer<T> {
    head: usize,
    tail: usize,
    buffer: Vec<T>,
    size: usize,
}

impl<T> CircularBuffer<T> where T: Clone { //richiede che T implementi Clone
    pub fn new(capacity: usize) -> Self {
        CircularBuffer{
            head: 0,
            tail: 0,
            buffer: Vec::with_capacity(capacity),
            size: capacity,
        }
    }

    pub fn write(&mut self, item: T) -> Result<(), String> {
        if self.tail != 0 && self.head == self.tail {
            return Err(format!("Buffer is full"));
        }
        else{
            self.buffer.insert(self.tail, item);
            self.tail += 1;
            if(self.tail >= self.size){
                self.tail = 0;
            }
        }
        Ok(())
    }

    pub fn read(&mut self) -> Option<T> {
        if (self.head == self.tail){
            None
        }else{
            let item = self.buffer[self.head].clone();
            self.head += 1;
            if(self.head >= self.size){
                self.head = 0;
            }
            Some(item)
        }
    }

    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.buffer.clear();
    }

    pub fn size(&self) -> usize{
        self.tail.abs_diff(self.head)
    }

    pub fn overwrite(&mut self, item: T) -> Result<(), String> {
        if self.head == self.tail {
            //buffer pieno
            self.buffer[self.tail] = item;
            self.tail += 1;
            self.head += 1;
            if(self.tail >= self.size){
                self.tail = 0;
            }
            if(self.head >= self.size){
                self.head = 0;
            }
        }
        else{
            self.buffer[self.tail] = item;
            self.tail += 1;
            if(self.tail >= self.size){
                self.tail = 0;
            }
        }
        Ok(())
    }

    pub fn make_contiguous(&mut self) ->() {
        if(self.head > self.tail){
            let mut new_buffer = Vec::with_capacity(self.size);
            for i in self.head..self.size{
                new_buffer.push(self.buffer.get(i).unwrap().clone());
            }
            for i in 0..self.tail{
                new_buffer.push(self.buffer.get(i).unwrap().clone());
            }
            self.buffer = new_buffer;
            self.tail = self.size();
            self.head = 0;
        }
    }
}