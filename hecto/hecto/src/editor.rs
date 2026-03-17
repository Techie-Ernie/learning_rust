use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::event::{read, Event::Key, KeyCode::Char};

// a struct is a group of variables and sometimes functions 
pub struct Editor {

}

impl Editor {
    pub fn default() -> Self {
        Editor{}
    }
    pub fn run(&self){
        // terminal gets hooked up to std input 
        // keyboard input gets read into b 
        // terminal starts in canonical mode (cooked mode) -> keyboard input is only sent when enter is presssed
        enable_raw_mode().unwrap();
        /* OLD 
        for b in io::stdin().bytes() {
            match b {
                Ok(b) => {
                    let c = b as char;
                    if c.is_control() {
                        // control characters: esp, backspace, etc
                        println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
                    }
                    else {
                        println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
                    }
                    if c == 'q' {
                        break;
                    }
                }
                Err(err) => println!("Error: {}", err),
            }
        }
        */ 

        // event is a KeyEvent: 
        /* 
        pub struct KeyEvent {
            pub code: KeyCode,
            pub modifiers: KeyModifiers,
            pub kind: KeyEventKind,
            pub state: KeyEventState,
        }
         */

        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{event:?}\r");
                    // if let is used for pattern matching where all other cases are ignored except for one 
                    if let Char(c) = event.code 
                        && c == 'q' {
                            break;
                        }
                },
                Err(err) => println!("Error: {err}"),
                _ => ()
            }
        }
        disable_raw_mode().unwrap();

    }
}