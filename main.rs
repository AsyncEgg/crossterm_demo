use std::{io::{stdout, Write}, thread, time};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}, Result
};

fn main() -> Result<()> {
  let mut stdout = stdout();
  stdout.execute(terminal::Clear(terminal::ClearType::All))?;
  stdout.execute(cursor::Hide)?;
  let x_len = 50;
  let y_len = 25;

  let mut bottom_hit = true;
  let mut right_hit = false;

  let mut y_pos = 10;
  let mut x_pos = 3;

  let mut x_old: u16 =1;
  let mut y_old: u16 =1;

  loop {
    for y in 0..y_len {
      for x in 0..x_len{
        if (y == 0 || y == y_len - 1) || (x == 0 || x == x_len - 1) {

          stdout
            .queue(cursor::MoveTo(x,y))?
            .queue(style::PrintStyledContent( "#".magenta()))?;
        }
      }
    }
    
    
    
    if y_pos >= y_len-1{
      bottom_hit = true;
    } else if y_pos <= 1 {
      bottom_hit = false;
    }

    if x_pos <= 1 {
      right_hit = true;
    } else if x_pos >= x_len-1 {
      right_hit = false;
    }

    if bottom_hit {
      y_pos -= 1
    } else {
      y_pos += 1
    }

    if right_hit {
      x_pos += 1
    } else {
      x_pos -= 1
    }
    
    x_old = x_pos;
    y_old = y_pos;

    stdout
      .queue(cursor::MoveTo(x_pos-1,y_pos-1))?
      .queue(style::PrintStyledContent("@".dark_green()))?;  

    stdout.flush()?;
    
    thread::sleep(time::Duration::from_millis(25));  
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
  }
  Ok(())
}