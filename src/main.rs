use std::{error::Error, io, time::{Duration, Instant}, sync::mpsc, thread};

use crossterm::{terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, cursor::{Hide, Show}, event::{self, Event, KeyCode}};
use invaders::{frame::{self, new_frame, Drawable}, render, audio, player::Player};
use rusty_audio::Audio;
use audio::register_audio;


fn main() -> Result<(), Box<dyn Error>>{
    println!("Hello, world!");

    let mut audio = Audio::new();
    let _ = register_audio(&mut audio);

    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render Loop in separate thread

    let (render_tx, render_rx) = mpsc::channel();
    
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };

            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    
    let mut player = Player::new();
    let mut instant = Instant::now();
    // Game loop
    'gameloop: loop {
        // Per frame nit
        let mut curr_frame = new_frame();
        let delta = instant.elapsed();
        instant = Instant::now();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }, 
                    KeyCode::Char('a') => player.move_left(),
                    KeyCode::Char('d') => player.move_right(),
                    KeyCode::Char('f') => player.shoot(),
                    _ => {}
                }
            }
        }

        player.update(delta);

        // Render
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    
    // Cleanup
    drop(render_tx);
    render_handle.join();
    audio.wait();
    Ok(())
}

