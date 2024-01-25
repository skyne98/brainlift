enum Command {
    IncrementPointer,
    DecrementPointer,
    IncrementData,
    DecrementData,
    Output,
    Input,
    LoopStart(usize),
    LoopEnd(usize),
}

fn parse_brainfuck(code: &str) -> Vec<Command> {
    let mut commands = Vec::new();
    let mut loop_stack = Vec::new();

    for (i, c) in code.chars().enumerate() {
        match c {
            '>' => commands.push(Command::IncrementPointer),
            '<' => commands.push(Command::DecrementPointer),
            '+' => commands.push(Command::IncrementData),
            '-' => commands.push(Command::DecrementData),
            '.' => commands.push(Command::Output),
            ',' => commands.push(Command::Input),
            '[' => {
                loop_stack.push(commands.len());
                commands.push(Command::LoopStart(0));
            }
            ']' => {
                if let Some(start_index) = loop_stack.pop() {
                    commands.push(Command::LoopEnd(start_index));
                    let commands_len = commands.len();
                    if let Command::LoopStart(ref mut end_index) = commands[start_index] {
                        *end_index = commands_len - 1;
                    }
                } else {
                    panic!("Unmatched ']' at position {}", i);
                }
            }
            _ => {} // Ignore other characters
        }
    }

    if !loop_stack.is_empty() {
        panic!("Unmatched '['");
    }

    commands
}

struct BrainfuckInterpreter {
    memory: Vec<u8>,
    pointer: usize,
}

impl BrainfuckInterpreter {
    fn new(initial_memory: Vec<u8>) -> Self {
        BrainfuckInterpreter {
            memory: initial_memory,
            pointer: 0,
        }
    }

    fn run(&mut self, code: &str) {
        let commands = parse_brainfuck(code);
        let mut pc = 0; // Program counter

        while pc < commands.len() {
            match commands[pc] {
                Command::IncrementPointer => {
                    self.pointer += 1;
                    if self.pointer >= self.memory.len() {
                        self.memory.push(0);
                    }
                }
                Command::DecrementPointer => {
                    if self.pointer == 0 {
                        panic!("Data pointer underflow");
                    }
                    self.pointer -= 1;
                }
                Command::IncrementData => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
                }
                Command::DecrementData => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
                }
                Command::Output => {
                    print!("{}", self.memory[self.pointer] as char);
                }
                Command::Input => {
                    // To handle input, you'd need to read from stdin or modify this to accept input another way
                    // For simplicity, this example does not handle input
                }
                Command::LoopStart(end) => {
                    if self.memory[self.pointer] == 0 {
                        pc = end;
                    }
                }
                Command::LoopEnd(start) => {
                    if self.memory[self.pointer] != 0 {
                        pc = start;
                    }
                }
            }
            pc += 1;
        }
    }
}

fn main() {
    let mut interpreter = BrainfuckInterpreter::new(vec![0; 30000]); // Initialize with 30,000 bytes of memory
    let code = "++[>++<-]>."; // Sample Brainfuck code
    interpreter.run(code);
}
