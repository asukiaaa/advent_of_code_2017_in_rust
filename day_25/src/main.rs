#[derive(Debug)]
struct TapeState {
    tape: Vec<u8>,
    start_index: usize,
    current_index: usize,
    current_state: char,
    step: u64,
}

impl TapeState {
    fn new() -> TapeState {
        let tape = vec![0];
        TapeState {
            tape: tape,
            start_index: 0,
            current_index: 0,
            current_state: 'a',
            step: 0,
        }
    }
    fn action(&mut self) {
        match self.current_state {
            'a' => self.action_a(),
            'b' => self.action_b(),
            'c' => self.action_c(),
            'd' => self.action_d(),
            'e' => self.action_e(),
            'f' => self.action_f(),
            _ => println!("error"),
        }
        self.step += 1;
    }
    fn action_a(&mut self) {
        // self.change_state_0_1(1, 'r', 'b',
        //                       0, 'l', 'b')
        self.change_state_0_1(1, 'r', 'b',
                              0, 'l', 'c')
    }
    fn action_b(&mut self) {
        // self.change_state_0_1(1, 'l', 'a',
        //                       1, 'r', 'a')
        self.change_state_0_1(1, 'l', 'a',
                              1, 'r', 'c')
    }
    fn action_c(&mut self) {
        self.change_state_0_1(1, 'r', 'a',
                              0, 'l', 'd')
    }
    fn action_d(&mut self) {
        self.change_state_0_1(1, 'l', 'e',
                              1, 'l', 'c')
    }
    fn action_e(&mut self) {
        self.change_state_0_1(1, 'r', 'f',
                              1, 'r', 'a')
    }
    fn action_f(&mut self) {
        self.change_state_0_1(1, 'r', 'a',
                              1, 'r', 'e')
    }
    fn change_state_0_1(&mut self, write0: u8, dir0: char, next_state0: char, write1: u8, dir1: char, next_state1: char) {
        if self.tape[self.current_index] == 0 {
            self.change_state(write0, dir0, next_state0)
        } else {
            self.change_state(write1, dir1, next_state1)
        }
    }
    fn change_state(&mut self, write: u8, dir: char, next_state: char) {
        self.tape[self.current_index] = write;
        if dir == 'r' {
            self.move_current_right();
        } else {
            self.move_current_left();
        }
        self.current_state = next_state;
    }
    fn move_current_right(&mut self) {
        self.current_index += 1;
        if self.current_index == self.tape.len() {
            let add_vec: Vec<u8> = vec![0; self.tape.len()];
            self.tape.extend(add_vec)
        }
    }
    fn move_current_left(&mut self) {
        if self.current_index == 0 {
            self.current_index = self.tape.len() - 1;
            self.start_index = self.start_index + self.tape.len();
            let mut add_vec: Vec<u8> = vec![0; self.tape.len()];
            add_vec.extend(self.tape.clone());
            self.tape = add_vec;
        } else {
            self.current_index -= 1
        }
    }
    fn is_breaking_time(&mut self) -> bool {
        //self.step == 6
        self.step == 12261543
    }
    fn count_of_one(&mut self) -> usize {
        let mut v = self.tape.clone();
        v.retain(|&i| i == 1);
        v.len()
    }
}

fn main() {
    let mut tape_state = TapeState::new();
    loop {
        tape_state.action();
        if tape_state.is_breaking_time() {
            break;
        }
    }
    //println!("{:?}", tape_state);
    println!("count of one {:?}", tape_state.count_of_one());
}
