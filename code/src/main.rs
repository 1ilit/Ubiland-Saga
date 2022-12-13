extern crate sdl2;

fn main() {
    println!("Hello, world!");

    let sdl = sdl2::init().unwrap();
    let video_subsystem=sdl.video().unwrap();
    let _window=video_subsystem.window("Ubiland Saga", 800, 600).resizable().build().unwrap();

    let mut event_pump=sdl.event_pump().unwrap();
    'main : loop{
        for event in event_pump.poll_iter(){
            //user input
            match event {
                sdl2::event::Event::Quit {..}=>break 'main,
                _ => {},
            }
        }

        //rendering
    }
}
