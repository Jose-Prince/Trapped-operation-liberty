mod framebuffer;
mod color;
mod fileReader;
mod bmp;
mod maze;
mod player;

use framebuffer::Framebuffer;
use color::Color;
use player::Player;
use maze::render;

fn main() {
    let width = 500;  // Ajusta el tamaño del framebuffer según sea necesario
    let height = 800; // Ajusta el tamaño del framebuffer según sea necesario
    let mut framebuffer = Framebuffer::new(width, height);

    match render(&mut framebuffer, "src/maze.txt") {
        Ok(Some((px, py))) => {
            println!("Maze rendered successfully");
            let player = Player::new(px as f32, py as f32);

            // Establecer el color para el jugador
            framebuffer.set_current_color(Color::new(255, 0, 0)); // Color rojo

            // Dibujar el jugador en el framebuffer
            framebuffer.point((player.pos.x) as isize, (player.pos.y) as isize);

            if let Err(e) = framebuffer.save_as_bmp("output.bmp") {
                eprintln!("Error saving BMP file: {}", e);
            }
        },
        Ok(None) => eprintln!("No player start point found in the maze"),
        Err(e) => eprintln!("Error rendering maze: {}", e),
    }
}
