use std::io;
use std::io::prelude::*;     
use std::{thread, time};


const BELL_CHARACTER: &str = "\x07";

macro_rules! loopn {
  ($n:expr, $body:block) => {
      for _ in 0..$n {
          $body
      }
  }
}


fn main() {
    let now = time::Instant::now();

    // TODO: configure this from the command line
    // 10 minutes as of now

    // TODO: if "-" read from stdin
    
    // TODO: allow for an absolute date as well as a timestamp 
    // TODO: (if in the past err)
    let countdown_seconds = 600;

    let countdown_duration = time::Duration::from_secs(countdown_seconds);
    thread::sleep(countdown_duration);

    // TODO: prompt every second with the remaining time
    // TODO: make this an option

    // TODO: make this an option
    let then = time::Instant::now();
    println!("[debug] missed by: {:?}", then - now - countdown_duration);

    // TODO: make this an option
    // TODO: allow customization of number of bells
    loopn! (3, {
        print!("{}", BELL_CHARACTER);
        io::stdout()
            .flush()
            .unwrap()
            ;

        thread::sleep(time::Duration::from_millis(500));
    });

    // TODO: allow arbitrary command execution - as an option
}
