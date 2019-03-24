#[cfg(test)]
mod main_test {
	use main::*;


	#[test]
	fn top_of_screen_test() {

    let mut app = App {
    	gl: GlGraphics::new(open_gl),
    	left_score: 0,
    	left_pos: 1,
    	left_vel: 0,
    	right_score: 0,
    	right_pos: 1,
    	right_vel: 0,
    	ball_x: 0,
    	ball_y: 0,
    	vel_x: 1,
    	vel_y: 1,
    };
    
	}

	#[test]
	fn botton_of_screen_test() {
		unimplemented!();
	}
}