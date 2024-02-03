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
    memory: Vec<i64>,
    pointer: usize,
}

impl BrainfuckInterpreter {
    fn new(initial_memory: Vec<i64>) -> Self {
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
                    print!("{}", self.memory[self.pointer])
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

fn add_one(x: i64) -> i64 {
    x + 1
}

fn codegen() {
    use cranelift::codegen::{
        control::ControlPlane,
        ir::{types::I64, AbiParam, Function, InstBuilder, Signature, UserFuncName},
        isa::CallConv,
        Context,
    };

    let mut sig = Signature::new(CallConv::SystemV);
    sig.params.push(AbiParam::new(I64));
    sig.returns.push(AbiParam::new(I64));

    let mut func = Function::with_name_signature(UserFuncName::default(), sig);
    use cranelift::frontend::{FunctionBuilder, FunctionBuilderContext};

    let mut func_ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut func, &mut func_ctx);
    let block = builder.create_block();
    builder.seal_block(block);

    builder.append_block_params_for_function_params(block);
    builder.switch_to_block(block);

    // Write the instructions
    let arg = builder.block_params(block)[0];
    let plus_one = builder.ins().iadd_imm(arg, 1);
    builder.ins().return_(&[plus_one]);

    builder.finalize();

    println!("{}", func.display());

    // Codegen
    use cranelift::codegen::{isa, settings};
    use target_lexicon::Triple;

    let builder = settings::builder();
    let flags = settings::Flags::new(builder);

    let isa = match isa::lookup(Triple::host()) {
        Err(err) => panic!("Error looking up target: {}", err),
        Ok(isa_builder) => isa_builder.finish(flags).unwrap(),
    };

    let mut ctx = Context::for_function(func);
    let mut control_plane = ControlPlane::default();
    let code = ctx.compile(&*isa, &mut control_plane).unwrap();

    // Map the compiled code and run
    let mut buffer = memmap2::MmapOptions::new()
        .len(code.buffer.data().len())
        .map_anon()
        .unwrap();

    buffer.copy_from_slice(code.code_buffer());

    let buffer = buffer.make_exec().unwrap();

    let x = unsafe {
        let code_fn: unsafe extern "sysv64" fn(usize) -> usize =
            std::mem::transmute(buffer.as_ptr());

        code_fn(1)
    };

    println!("out: {}", x);
}

fn main() {
    let mut interpreter = BrainfuckInterpreter::new(vec![0; 30000]); // Initialize with 30,000 bytes of memory
    let code = "++[>++<-]>."; // Sample Brainfuck code
    interpreter.run(code);

    // Now try the code compilation and running
    codegen();
}
