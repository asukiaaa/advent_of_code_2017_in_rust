#[derive(Debug)]
struct TapeState {
    tape: Vec<u8>,
    start_index: usize,
    current_index: usize,
    step: u64,
}

impl TapeState {
    fn new() -> TapeState {
        let tape = vec![0];
        TapeState {
            tape: tape,
            start_index: 0,
            current_index: 0,
            step: 0,
        }
    }
    fn action(&mut self) {
        if self.step % 2 == 0 {
            self.action_a();
        } else {
            self.action_b();
        }
        self.step += 1;
    }
    fn action_a(&mut self) {
        if self.tape[self.current_index] == 0 {
            self.tape[self.current_index] = 1;
            self.move_current_right();
        } else {
            self.tape[self.current_index] = 0;
            self.move_current_left();
        }
    }
    fn action_b(&mut self) {
        if self.tape[self.current_index] == 0 {
            self.tape[self.current_index] = 1;
            self.move_current_left();
        } else {
            self.move_current_right();
        }
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
        self.current_index == self.start_index &&
            self.tape[self.current_index] == 0
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
    println!("count of one {:?}", tape_state.count_of_one());
    println!("{:?}", tape_state);
}
