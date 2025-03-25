#[derive(Debug)]
pub struct BFInt {
    pub prog: Vec<u8>,
    pub prog_ptr: usize,
    pub mem: Vec<u8>,
    pub mem_ptr: usize,
    pub loop_map: Vec<(usize, usize)>,
}

impl BFInt {
    pub fn new() -> BFInt {
        BFInt {
            prog: Vec::new(),
            prog_ptr: 0,
            mem: vec![0; 1000],
            mem_ptr: 0,
            loop_map: Vec::new(),
        }
    }

    pub fn extend_prog(&mut self, new_prog: &[u8]) {
        self.prog.extend_from_slice(new_prog);
        self.extend_loop_map();
    }

    fn extend_loop_map(&mut self) {
        // would be better to keep the loop map as a sorted array based on the source index
        let mut pc = self.prog_ptr;
        let mut start_stack: Vec<usize> = Vec::new();
        while pc < self.prog.len() {
            match self.prog[pc] {
                b'[' => start_stack.push(pc + self.prog_ptr),
                b']' => self
                    .loop_map
                    .push((start_stack.pop().unwrap(), self.prog_ptr + pc)),
                _ => {}
            }
            pc += 1;
        }
    }

    fn ensure_allocated(&mut self, index: usize) {
        if index >= self.mem.len() {
            // probablly very inneficient
            self.mem.resize(index, 0);
        }
    }

    pub fn step(&mut self) {
        if self.prog_ptr >= self.prog.len() {
            return;
        }

        match self.prog[self.prog_ptr] {
            b'>' => self.mem_ptr += 1,
            b'<' => self.mem_ptr -= 1,
            b'+' => self.mem[self.mem_ptr] = self.mem[self.mem_ptr].wrapping_add(1),
            b'-' => self.mem[self.mem_ptr] = self.mem[self.mem_ptr].wrapping_sub(1),
            b'.' => print!("{}", self.mem[self.mem_ptr] as char),
            b',' => todo!(),
            b'[' => {
                if self.mem[self.mem_ptr] == 0 {
                    self.prog_ptr = self
                        .loop_map
                        .iter()
                        .find(|&(s, _)| *s == self.prog_ptr)
                        .unwrap()
                        .1;
                }
            }
            b']' => {
                if self.mem[self.mem_ptr] != 0 {
                    self.prog_ptr = self
                        .loop_map
                        .iter()
                        .find(|&(_, d)| *d == self.prog_ptr)
                        .unwrap()
                        .0;
                }
            }
            _ => {} // ignore all non-relevant bytes
        }
        self.prog_ptr += 1;
    }

    pub fn run(&mut self) {
        while self.prog_ptr < self.prog.len() {
            self.step();
        }
    }
}
