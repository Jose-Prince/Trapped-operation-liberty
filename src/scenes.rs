use crate::AudioPlayer;
use crate::Framebuffer;
use crate::Color;

use std::time::{Duration, Instant};
use minifb::{Window, Key};

pub fn game_start(width: usize, height: usize, framebuffer: &mut Framebuffer, window: &mut Window) {
    let mut audio = AudioPlayer::new("Audio/Inicio.mp3");
    
    audio.play();

    let begin_page = "textures/Inicio.png";
    
    let blink_interval = Duration::from_millis(200);
    let mut last_blink_time = Instant::now();
    let mut show_text = true;

    while window.is_open() && !window.is_key_down(minifb::Key::Enter) && !window.is_key_down(minifb::Key::Escape) {
        framebuffer.clear();
        framebuffer.draw_image(&begin_page, width, height);
        
        if last_blink_time.elapsed() >= blink_interval {
            show_text = !show_text;
            last_blink_time = Instant::now();
        }

        if show_text {
            framebuffer.draw_text(width/5,(4*height)/5 - 25,"Press ENTER to start game", Color::new(255,255,255), 70.0);
        }

        window.update_with_buffer(&framebuffer.get_buffer(), width, height).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }

    audio.stop();

}

pub fn level_selector() {

}

pub fn gameplay() {

}

pub fn win_screen() {

}

pub fn defeat_screen() {

}