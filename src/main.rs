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
    // 10 minutes as of now - default
    // add a s,m,h,d,w,y parser - default to s if cannot recognize

    // TODO: if "-" read from stdin
    
    let countdown_seconds = 600;

    let countdown_duration = time::Duration::from_secs(countdown_seconds);
    //thread::sleep(countdown_duration);

    // TODO: make this an option
    // TODO: make sure as little time is lost as possible
    // TODO: allow for overwriting the output (option)
    // TODO: allow custom command here
    for i in (0..countdown_seconds).rev() {
        thread::sleep(time::Duration::from_secs(1));

        // TODO: extract const
        // TODO: can this be made generic? / do it up to years
        let minutes_remaining = i / 60;
        let seconds_remaining = i % 60;

        let formatted_minutes = if minutes_remaining > 0 { format!("{}m ", minutes_remaining) } else { "".to_string() };
        let formatted_seconds = format!("{}s", seconds_remaining);

        println!("{}{}", formatted_minutes, formatted_seconds);
    }

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
    // TODO: test with dunst integration
}
