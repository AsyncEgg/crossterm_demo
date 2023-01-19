use std::{io::{stdout, Write,Stdout}, thread, time};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}, Result
};

struct Point {
    x: u16,
    y: u16,
}

fn main() -> Result<()> {
    let CSL_SIZE_X = 50;
    let CSL_SIZE_Y = 25;

    let mut ball = Point{x:10, y:10};
    let mut old_ball_pos = Point{x:10, y:10};
    
    let mut move_ball_up = false;
    let mut move_ball_left = true;
    
    let mut stdout = stdout();
    
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::Hide)?;
    //draws rectangle
    loop {
      for y in 0..CSL_SIZE_Y {
        for x in 0..CSL_SIZE_X{
          if (y == 0 || y == CSL_SIZE_Y - 1) || (x == 0 || x == CSL_SIZE_X - 1) {
  
            stdout
              .queue(cursor::MoveTo(x,y))?
              .queue(style::PrintStyledContent( "#".magenta()))?;
          }
        }
      }
      //clear previous ball pos
      stdout
        .queue(cursor::MoveTo(old_ball_pos.x-1,old_ball_pos.y-1))?
        .queue(style::Print(" "))?;
      
      if ball.y >= CSL_SIZE_Y - 1 { move_ball_up = true;}
        else if ball.y <= 2 { move_ball_up = false;}
  
      if ball.x <= 2 { move_ball_left = true;} 
        else if ball.x >= CSL_SIZE_X - 1 { move_ball_left = false;}
  
      if move_ball_up { ball.y -= 1 }
        else if !move_ball_up { ball.y += 1 }
  
      if move_ball_left { ball.x += 1 }
        else if !move_ball_left { ball.x -= 1 }
      
      old_ball_pos.x = ball.x;
      old_ball_pos.y = ball.y;
  
      stdout
        .queue(cursor::MoveTo(ball.x-1,ball.y-1))?
        .queue(style::PrintStyledContent("@".dark_green()))?;  
      
      stdout.flush()?;
      //thread::sleep(time::Duration::from_millis(10));

      //just to remove warning will fix later
        if ball.x == 100 && ball.y == 100{
          break
        }
      }
    Ok(())
}
