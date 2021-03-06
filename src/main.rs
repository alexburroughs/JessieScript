use std::env;
use std::fs::File;
use std::io::prelude::*;

const CHARS: [&'static str; 96] = [" ","!","\"","#","$","%",
"&","'","(",")","*","+",",","-",".","/","0","1","2","3","4",
"5","6","7","8","9",":",";","<","=",">","?","@","A","B","C",
"D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R",
"S","T","U","V","W","X","Y","Z","[", "\\" ,"]","^","_","`",
"a","b","c","d","e","f","g","h","i","j","k","l","m","n","o",
"p","q","r","s","t","u","v","w","x","y","z","{","|","}","~",
"",];

fn main() {

    // Get arguments (source code path)
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("please provide a filename as an argument");
    }

    let filename = &args[1];

    // Open file
    let mut f = File::open(filename).expect("file not found");

    // Read all source code into a string
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("error reading file");

    // Hack to make parsing easier
    contents.push(' ');

    let tokens = parse_file(contents);

    run(tokens);
}

fn parse_file(code : String) -> Vec<Token> {

    use State::*;

    // Get source code string as chars
    let chars : Vec<char> = code.chars().collect();

    // Empty list of tokens to be populated and returned
    let mut tokens : Vec<Token> = Vec::new();

    // Counter for loop, line counter, id and current state
    let mut counter = 0;
    let mut line = 1;
    let mut id = 0;
    let mut state = State::START;

    // Current text being processed
    let mut current = String::from("");

    // loop through all chars in the source code
    while counter < chars.len() {

        if let Some(ref tmp) = chars.get(counter) {

            match state {
                START => {

                    let curr = tmp.clone();

                    if *curr == '/' {
                        state = COMMENT;
                    }
                    else if tmp.is_digit(10) || *curr == '-' {

                        // temporary clone of tmp so it can be used TODO: maybe delete?
                        let curr = tmp.clone();
                        current.push(*curr);

                        // set the state
                        state = NUMBER;
                    }
                    else if tmp.is_whitespace() {

                        // TODO delete and see what happens
                    }
                    else if tmp.is_ascii_alphabetic() {
                        
                        // push the current char to the current token
                        let curr = tmp.clone();
                        current.push(*curr);

                        // set the state
                        state = KEY;
                    }

                    else {
                        panic!("Error in parsing");
                    }
                },
                
                NUMBER => {
                    if tmp.is_digit(10) {

                        // Continue pushing chars onto the current token
                        let curr = tmp.clone();
                        current.push(*curr);
                    }

                    // If the current token is done
                    else if tmp.is_whitespace() {

                        // Make the token and push onto tokens list
                        let curr_token = Token {
                            id : id,
                            key : Keyword::NUMBER,
                            text : current.clone(),
                            line : line
                        };

                        //println!("line: {} text: {} id: {}\n", &line, &curr_token.text, &id);
                        tokens.push(curr_token);

                        // Reset the current token and state
                        current = String::new();
                        state = START;

                        line += 1;
                        id += 1;
                    }

                    // Error in source code (garbage programmer)
                    else {
                        panic!("Error in parsing");
                    }
                },
                KEY => {

                    let curr = tmp.clone();

                    if tmp.is_ascii_alphabetic() {

                        // Continue pushing chars onto the current token
                        let curr = tmp.clone();
                        current.push(*curr);
                    }

                    // If the current token is done
                    else if tmp.is_whitespace() {

                        let token_type : Keyword;

                        let mut line_add = 0;

                        // Make the token and push onto tokens list
                        // set if the line will be incremented based on the 
                        match current.as_ref() {
                            "push" => {token_type = Keyword::PUSH},
                            "pop" => {token_type = Keyword::POP; line_add += 1},
                            "add" => {token_type = Keyword::ADD; line_add += 1},
                            "ifeq" => {token_type = Keyword::IFEQ; line_add += 1},
                            "jump" => {token_type = Keyword::JUMP},
                            "print" => {token_type = Keyword::PRINT; line_add += 1},
                            "dup" => {token_type = Keyword::DUP; line_add += 1},
                            "save" => {token_type = Keyword::SAVE},
                            "restore" => {token_type = Keyword::RESTORE},
                            "printc" => {token_type = Keyword::PRINTC}
                            _ => {token_type = Keyword::ADDRESS}
                        }

                        // Make the token and push onto tokens list
                        let curr_token = Token {
                            id : id,
                            key : token_type,
                            text : current.clone(),
                            line : line
                        };

                        tokens.push(curr_token);

                        // Reset the current token and state
                        current = String::new();
                        state = START;

                        line += line_add;
                        id += 1;
                    }

                    else if *curr == ':' {
                        
                        // Make the token and push onto tokens list
                        let curr_token = Token {
                            id : id,
                            key : Keyword::ADDRESSKEY,
                            text : current.clone(),
                            line : line
                        };

                        tokens.push(curr_token);

                        // Reset the current token and state
                        current = String::new();
                        state = START;

                        line += 1;
                        id += 1;
                    }

                    // Error in source code (garbage programmer)
                    else {
                        panic!("Error in parsing");
                    }
                }

                COMMENT => {
                    
                    let curr = tmp.clone();

                    if *curr == '/' {
                        state = START;
                    }
                }
            }
        }

        counter += 1;
    }
    return tokens;
}

fn run (tokens : Vec<Token>) {

    use Keyword::*;

    let mut num_stack : Vec<i32> = Vec::new();
    let mut memory = [0; 1024];

    let mut current_id = 0;

    while current_id < tokens.len() {

        // Stuff I have to do but dont like
        if let Some(ref token) = tokens.get(current_id) {

            match &token.key {
                PUSH => {

                    current_id += 1;

                    if let Some(ref num) = tokens.get(current_id) {
                        num_stack.push(num.text.parse::<i32>().unwrap());
                    }
                    current_id += 1;
                },
                POP => {
                    if num_stack.len() < 1{
                        panic!("Runtime Error: too much popping");
                    }
                    
                    num_stack.pop();

                    current_id += 1;
                },
                ADD => {
                    //println!("len: {}", num_stack.len());
                    if num_stack.len() < 2 {
                        panic!("Runtime Error: too much popping");
                    }

                    let x : i32;
                    let y : i32;

                    match num_stack.pop() {
                        Some(ref a) => {x = *a},
                        None => panic!()
                    }

                    match num_stack.pop() {
                        Some(ref a) => {y = *a},
                        None => panic!()
                    }

                    num_stack.push(x + y);
                    
                    current_id += 1;
                },
                IFEQ => {

                    if num_stack.len() < 1 {
                        panic!("Runtime Error: too much popping");
                    }

                    let x : i32;

                    match num_stack.pop() {
                        Some(ref a) => {x = *a},
                        None => panic!()
                    }
                    
                    num_stack.push(x);

                    if x == 0 {
                        current_id += 2;
                    }
                    else {
                        current_id += 1;
                        // Get the address token
                        if let Some(ref num) = tokens.get(current_id) {
                            match num.key {
                            Keyword::ADDRESS => {
                                current_id = get_address_name(&num.text, &tokens);
                            },

                            Keyword::NUMBER => {
                                current_id = get_address(num.text.parse::<i32>().unwrap(), &tokens);
                            },
                            _ => panic!("at the disco")
                        }
                        }
                    }
                    continue;
                },
                JUMP => {
                    current_id += 1;
                    // Get the address token
                    if let Some(ref num) = tokens.get(current_id) {

                        match num.key {
                            Keyword::ADDRESS => {
                                current_id = get_address_name(&num.text, &tokens);
                            },

                            Keyword::NUMBER => {
                                current_id = get_address(num.text.parse::<i32>().unwrap(), &tokens);
                            },
                            _ => panic!("at the disco")
                        }
                    }

                    continue;
                },
                PRINT => {
                    if num_stack.len() < 1{
                        panic!("Runtime Error: not enough pushing");
                    }
                    
                    // TODO replace
                    // LAZY CODE ----
                    //              V

                    let x : i32;

                    match num_stack.pop() {
                        Some(ref a) => {x = *a},
                        None => panic!()
                    }

                    println!("{}", x);

                    num_stack.push(x);

                    // END OF LAZY CODE
                    current_id += 1;
                },
                PRINTC => {

                    current_id+=1;
                    if let Some(ref num) = tokens.get(current_id) {

                        let mut tmp_stack : Vec<i32> = Vec::new();
                        let parsed_num = num.text.parse::<u32>().unwrap();
                        for _x in 0..parsed_num {
                            if num_stack.len() < 1{
                                panic!("Runtime Error: not enough pushing");
                            }

                            let x : i32;
                            match num_stack.pop() {
                                Some(ref a) => {x = *a},
                                None => panic!()
                            }

                            print!("{}", get_char_code(x));


                            tmp_stack.push(x);
                        }
                        println!();

                        for _x in 0..parsed_num {
                            let x : i32;

                            match tmp_stack.pop() {
                            Some(ref a) => {x = *a},
                            None => panic!()
                            }

                            num_stack.push(x);
                        }
                    }

                    current_id += 1;
                },
                DUP => {
                    if num_stack.len() < 1{
                        panic!("Runtime Error: not enough pushing");
                    }
                    
                    // TODO replace
                    // MORE LAZY CODE ----
                    //                   V

                    let x : i32;

                    match num_stack.pop() {
                        Some(ref a) => {x = *a},
                        None => panic!()
                    }

                    num_stack.push(x);
                    num_stack.push(x);

                    // END OF LAZY CODE
                    current_id += 1;
                },
                SAVE => {
                    current_id += 1;
                    // Get the address token
                    if let Some(ref num) = tokens.get(current_id) {
                        let parsed_num = num.text.parse::<u32>().unwrap();
                        if parsed_num >= 1024 {
                            panic!("Runtime Error: cannot save to address. Must be between 0-1023 inc.");
                        }

                        memory[parsed_num as usize] = num_stack.pop().unwrap();
                    }

                    current_id += 1;
                },
                RESTORE => {
                    current_id += 1;
                    // Get the address token
                    if let Some(ref num) = tokens.get(current_id) {
                        let parsed_num = num.text.parse::<u32>().unwrap();
                        if parsed_num >= 1024 {
                            panic!("Runtime Error: cannot restore from address. Must be between 0-1023 inc.");
                        }

                        num_stack.push(memory[parsed_num as usize]);
                    }

                    current_id += 1;
                },
                ADDRESSKEY => {
                    current_id += 1;
                }
                _ => panic!("Runtime Error: oh no, a bug in the interpreter")
            }
        }
    } 
}

fn get_address(address: i32, tokens : &Vec<Token>) -> usize {

    for x in tokens {
        match x.key {
            Keyword::NUMBER => continue,
            _ => {
                if x.line == address {
                    return x.id;
                }
            }
        }
    }

    panic!("Runtime Error: invalid line number (lines start at 1)");
}

fn get_address_name(address: &String, tokens: &Vec<Token>) -> usize {

    for x in tokens {
        match x.key {
            Keyword::ADDRESSKEY => {
                if x.text == *address {
                    return x.id;
                }
            },
            _ => continue
        }
    }

    panic!("Runtime Error: invalid line number (lines start at 1)");
}

fn get_char_code(c: i32) -> &'static str {
    CHARS.get((c - 32) as usize).unwrap()
}

struct Token {
    id : usize,
    key : Keyword,
    text : String,
    line : i32
}

enum Keyword {
    PUSH,
    POP,
    ADD,
    IFEQ,
    JUMP,
    PRINT,
    DUP,
    NUMBER,
    SAVE,
    RESTORE,
    ADDRESS,
    ADDRESSKEY,
    PRINTC
}

enum State {
    START,
    NUMBER,
    KEY,
    COMMENT
}