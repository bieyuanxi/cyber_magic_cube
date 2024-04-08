
#[derive(Debug, Clone, Copy)]
struct Data;

struct Stack<const N: usize> {
    buf: [Data; N],
    botton: usize,
    top: usize,
    cached: usize,
}

impl<const N: usize> Stack<N> {
    fn new() -> Stack<N> {
        Self {
            buf: [Data; N],
            botton: 0,
            top: 0,
            cached: 0,
        }
    }

    fn full(&self) -> bool {
        (self.top + 1) % N == self.botton
    }

    fn empty(&self) -> bool {
        self.top == self.botton
    }

    fn push(&mut self, data: Data) {
        self.buf[self.top] = data;
        self.top += 1;
        self.top %= N;
        self.cached = self.top; // sync

        if self.full() {
            self.botton += 1;
            self.botton %= N;
        }
    }

    fn pop(&mut self) {
        if !self.empty() {
            self.top += N - 1;
            self.top %= N;
        }
    }
}