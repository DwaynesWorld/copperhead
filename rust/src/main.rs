mod snake;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    terminal::{self, ClearType},
    ExecutableCommand, Result,
};

use rand::Rng;
use snake::{Direction, Position, Snake};
use std::{
    io::{stdout, Write},
    thread,
    time::{Duration, Instant},
};

fn main() -> Result<()> {
    // Prepare terminal in raw mode and switch to the alternate screen.
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    // Get terminal dimensions.
    let (width, height) = terminal::size()?;

    // Initialize snake in the center of the screen.
    let start_position = Position {
        x: width / 2,
        y: height / 2,
    };
    let mut snake = Snake::new(start_position);

    // Place the first food item randomly.
    let mut rng = rand::thread_rng();
    let mut food = Position {
        x: rng.gen_range(1..width - 1),
        y: rng.gen_range(1..height - 1),
    };

    // Game settings.
    let tick_rate = Duration::from_millis(75);
    let mut last_tick = Instant::now();
    let mut game_over = false;

    // Game loop.
    while !game_over {
        // Handle input (non-blocking).
        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => snake.change_direction(Direction::Up),
                    KeyCode::Down => snake.change_direction(Direction::Down),
                    KeyCode::Left => snake.change_direction(Direction::Left),
                    KeyCode::Right => snake.change_direction(Direction::Right),
                    KeyCode::Char('q') => break, // Quit game on 'q'
                    _ => {}
                }
            }
        }

        // Update game state on each tick.
        if last_tick.elapsed() < tick_rate {
            // Brief sleep to reduce CPU usage.
            thread::sleep(Duration::from_millis(1));
            continue;
        }

        // Determine whether the snake should grow (when it eats the food).
        let grow = snake.head() == food;
        snake.move_forward(grow);

        if grow {
            // Place new food in a free location.
            loop {
                let new_food = Position {
                    x: rng.gen_range(1..width - 1),
                    y: rng.gen_range(1..height - 1),
                };
                if !snake.is_colliding(&new_food) {
                    food = new_food;
                    break;
                }
            }
        }

        // Check for collisions with the wall.
        let head = snake.head();
        if head.x == 0 || head.x == width - 1 || head.y == 0 || head.y == height - 1 {
            game_over = true;
        }

        // Check for self-collision.
        if snake.check_self_collision() {
            game_over = true;
        }

        // Draw everything.
        stdout.execute(terminal::Clear(ClearType::All))?;

        // Draw border.
        for x in 0..width {
            stdout.execute(cursor::MoveTo(x, 0))?;
            print!("#");
            stdout.execute(cursor::MoveTo(x, height - 1))?;
            print!("#");
        }
        for y in 0..height {
            stdout.execute(cursor::MoveTo(0, y))?;
            print!("#");
            stdout.execute(cursor::MoveTo(width - 1, y))?;
            print!("#");
        }

        // Draw the food.
        stdout.execute(cursor::MoveTo(food.x, food.y))?;
        print!("*");

        // Draw the snake.
        for (i, pos) in snake.body.iter().enumerate() {
            stdout.execute(cursor::MoveTo(pos.x, pos.y))?;
            if i == 0 {
                print!("O"); // Head
            } else {
                print!("o"); // Body
            }
        }

        stdout.flush()?;
        last_tick = Instant::now();

        // Brief sleep to reduce CPU usage.
        thread::sleep(Duration::from_millis(1));
    }

    // Display a "Game Over" message.
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(width / 2 - 5, height / 2))?;
    println!("Game Over!");
    stdout.flush()?;
    thread::sleep(Duration::from_secs(2));

    // Restore terminal state.
    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
