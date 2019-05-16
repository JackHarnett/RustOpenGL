use glium::{glutin};
use glium::backend::glutin::DisplayCreationError;
use std::{thread, time};

pub struct Game<T : AbstractGameLogic> {
	pub display : glium::Display,
	pub event_loop : glutin::EventsLoop,
	pub game_logic : T,
}

impl<X : AbstractGameLogic> Game<X> {

	// Initialise the game window and event loop. 
	pub fn new(logic : X) -> Result<Self, DisplayCreationError> {
	    let event_loop = glutin::EventsLoop::new();
    	let window_builder = glutin::WindowBuilder::new();
    	let context_builder = glutin::ContextBuilder::new();

    	let disp = glium::Display::new(window_builder, context_builder, &event_loop) ;

		disp.and_then(|window : glium::Display| {
    		Ok(Game {
    			display : window,
    			event_loop : event_loop,
    			game_logic : logic,
    		})
    	})
	}

	pub fn init(&mut self) {
		loop {
			self.game_logic.handle_input(&mut self.event_loop);
			self.game_logic.update();
			self.game_logic.draw(&mut self.display);
		
			if(self.game_logic.window_should_close()) {
				return;
			} ;
		}
	}
}

pub trait AbstractGameLogic {

	fn handle_input(&mut self, event_loop : &mut glutin::EventsLoop);

	fn update(&mut self);

	fn draw(&mut self, window : &mut glium::Display);

	fn window_should_close(&self) -> bool;

	fn close(&mut self);

}

pub struct GameLogic { 
	pub should_close : bool,
}

impl AbstractGameLogic for GameLogic {

	fn handle_input(&mut self, event_loop : &mut glutin::EventsLoop) {

        use glutin::{Event, WindowEvent};

		event_loop.poll_events(|event| { 
			match event {
				Event::WindowEvent {event, ..} => {
					if(event == WindowEvent::CloseRequested) {
						self.close();
					}
				},
				_ => {},
			}
		});
	}

	fn update(&mut self) {}

	fn draw(&mut self, window : &mut glium::Display) {}

	fn window_should_close(&self) -> bool { 
		self.should_close
	}

	fn close(&mut self) {
		self.should_close = true
	}
}