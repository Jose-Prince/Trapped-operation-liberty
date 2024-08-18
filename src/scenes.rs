use crate::AudioPlayer;
use crate::Framebuffer;
use crate::Color;
use crate::polygon::Polygon;

use std::time::{Duration, Instant};
use minifb::{Window, Key};

pub fn game_start(width: usize, height: usize, framebuffer: &mut Framebuffer, window: &mut Window) {
    let mut audio = AudioPlayer::new("Audio/Inicio.mp3");
    
    audio.play();

    let begin_page = "textures/Inicio.png";
    
    let blink_interval = Duration::from_millis(200);
    let mut last_blink_time = Instant::now();
    let mut show_text = true;
    let mut enter_pressed = false;

    while window.is_open() && !enter_pressed && !window.is_key_down(minifb::Key::Escape) {
        framebuffer.clear();
        framebuffer.draw_image(&begin_page, width, height);
        
        if last_blink_time.elapsed() >= blink_interval {
            show_text = !show_text;
            last_blink_time = Instant::now();
        }

        if show_text {
            framebuffer.draw_text(width / 5, (4 * height) / 5 - 25, "Press ENTER to start game", Color::new(255, 255, 255), 70.0);
        }

        // Detectar cuando Enter se presiona por primera vez
        if window.is_key_down(minifb::Key::Enter) {
            enter_pressed = true;
        }

        window.update_with_buffer(&framebuffer.get_buffer(), width, height).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }

    // Ahora esperar a que se suelte la tecla Enter
    while enter_pressed && window.is_open() {
        framebuffer.clear();

        // Continuar solo cuando la tecla Enter se ha soltado
        if !window.is_key_down(minifb::Key::Enter) {
            enter_pressed = false;
        }

        window.update_with_buffer(&framebuffer.get_buffer(), width, height).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }

    audio.stop();

    level_selector(framebuffer, window, width, height);
}

pub fn level_selector(framebuffer: &mut Framebuffer, window: &mut Window, width: usize, height: usize) {
    let mut option = 0;

    // Definición de los polígonos de fondo para cada nivel
    let first_level_background: Vec<[isize; 2]> = vec![
        [0, 0],
        [0, height as isize],
        [width as isize / 3, height as isize],
        [width as isize / 3, 0],
    ];

    let second_level_background: Vec<[isize; 2]> = vec![
        [width as isize / 3, 0],
        [width as isize / 3, height as isize],
        [2 * (width as isize) / 3, height as isize],
        [2 * (width as isize) / 3, 0],
    ];

    let third_level_background: Vec<[isize; 2]> = vec![
        [2 * (width as isize) / 3, 0],
        [2 * (width as isize) / 3, height as isize],
        [width as isize, height as isize],
        [width as isize, 0],
    ];

    // Bucle principal de la selección de nivel
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        framebuffer.clear();

        // Dibujar el fondo según la opción seleccionada
        match option {
            0 => framebuffer.polygon(&first_level_background, Color::new(128, 128, 128), Color::new(128, 128, 128)),
            1 => framebuffer.polygon(&second_level_background, Color::new(128, 128, 128), Color::new(128, 128, 128)),
            2 => framebuffer.polygon(&third_level_background, Color::new(128, 128, 128), Color::new(128, 128, 128)),
            _ => println!("Invalid option"),
        }

        // Cambiar opción con las teclas de flecha
        if window.is_key_down(minifb::Key::Left) && option > 0 {
            option -= 1;
        } else if window.is_key_down(minifb::Key::Right) && option < 2 {
            option += 1;
        }

        // Dibujar los textos y las imágenes
        framebuffer.draw_text(width / 24 + 40, height / 5, "Level 1", Color::new(255, 255, 255), 60.0);
        framebuffer.draw_image_at_position("textures/prison1.jpeg", width / 4, height / 4, width / 24 - 1, 2 * height / 5);

        framebuffer.draw_text(2 * width / 5 + 30, height / 5, "Level 2", Color::new(255, 255, 255), 60.0);
        framebuffer.draw_image_at_position("textures/prison2.jpg", width / 4, height / 4, 2 * width / 5 - 25, 2 * height / 5);

        framebuffer.draw_text(4 * width / 6 + 85, height / 5, "Level 3", Color::new(255, 255, 255), 60.0);
        framebuffer.draw_image_at_position("textures/prison3.jpg", width / 4, height / 4, 4 * width / 6 + 45, 2 * height / 5);

        framebuffer.draw_text(width / 3 + 5, height / 30, "Level selector", Color::new(255, 255, 255), 60.0);

        // Salir del ciclo si se presiona Enter
        if window.is_key_down(minifb::Key::Enter) {
            break;
        }

        // Actualizar la ventana con el contenido del framebuffer
        window.update_with_buffer(&framebuffer.get_buffer(), width, height).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }
}


pub fn gameplay() {

}

pub fn win_screen() {

}

pub fn defeat_screen() {

}