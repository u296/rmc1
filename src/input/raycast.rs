
/// marches a ray in the given direction and returns the block it hits
pub fn raycast<F>(check_coordinate_occupied: F, start_position: [f32; 3], direction: [f32; 3], max_distance: f32) -> Option<[i32; 3]> 
    where F: Fn([i32; 3]) -> bool {
        

        let unit_step_size = [
            (1.0 + (direction[1] / direction[0]).powi(2) + (direction[2] / direction[0]).powi(2)).sqrt(),
            (1.0 + (direction[0] / direction[1]).powi(2) + (direction[2] / direction[1]).powi(2)).sqrt(),
            (1.0 + (direction[0] / direction[2]).powi(2) + (direction[1] / direction[2]).powi(2)).sqrt(),
		];

        let mut current_block = [
            start_position[0].floor() as i32,
            start_position[1].floor() as i32,
            start_position[2].floor() as i32,
		];

        // check if starting point is already in block
        if check_coordinate_occupied(current_block) {
            return Some(current_block);
        }

		let mut travelled_ray: [f32; 3] = Default::default();
		let mut step: [i32; 3] = Default::default();

		// Establish Starting Conditions
		if direction[0] < 0.0
		{
			step[0] = -1;
			travelled_ray[0] = (start_position[0] - current_block[0] as f32) * unit_step_size[0];
		}
		else
		{
			step[0] = 1;
			travelled_ray[0] = ((current_block[0] + 1) as f32 - start_position[0]) * unit_step_size[0];
		}

		if direction[1] < 0.0
		{
			step[1] = -1;
			travelled_ray[1] = (start_position[1] - current_block[1] as f32) * unit_step_size[1];
		}
		else
		{
			step[1] = 1;
			travelled_ray[1] = ((current_block[1] + 1) as f32 - start_position[1]) * unit_step_size[1];
		}

        if direction[2] < 0.0
		{
			step[2] = -1;
			travelled_ray[2] = (start_position[2] - current_block[2] as f32) * unit_step_size[2];
		}
		else
		{
			step[2] = 1;
			travelled_ray[2] = ((current_block[2] + 1) as f32 - start_position[2]) * unit_step_size[2];
		}

		// Perform "Walk" until collision or range check
		
		let mut total_distance = 0.0;

		while total_distance < max_distance
		{
			// Walk along shortest path
			if travelled_ray[0] < travelled_ray[1] && travelled_ray[0] < travelled_ray[2]
			{
				current_block[0] += step[0];
				total_distance = travelled_ray[0];
				travelled_ray[0] += unit_step_size[0];
			}
			else if travelled_ray[1] < travelled_ray[2]
			{
				current_block[1] += step[1];
				total_distance = travelled_ray[1];
				travelled_ray[1] += unit_step_size[1];
			}
            else {
                current_block[2] += step[2];
                total_distance = travelled_ray[2];
                travelled_ray[2] += unit_step_size[2];
            }

			// Test tile at new test point
            if check_coordinate_occupied(current_block) {
                return Some(current_block);
            }
        }

        None
}

/// marches a ray in the given direction and returns a list of all the block spaces it travelled through
/// INCLUDING the one that it hit. The bool indicates if the last block in the list was a block that
/// was hit.
/// 
/// This function is guaranteed to return at least 1 block
pub fn raycast_path<F>(check_coordinate_occupied: F, start_position: [f32; 3], direction: [f32; 3], max_distance: f32) -> (bool, Vec<[i32; 3]>) 
    where F: Fn([i32; 3]) -> bool {
        

        let unit_step_size = [
            (1.0 + (direction[1] / direction[0]).powi(2) + (direction[2] / direction[0]).powi(2)).sqrt(),
            (1.0 + (direction[0] / direction[1]).powi(2) + (direction[2] / direction[1]).powi(2)).sqrt(),
            (1.0 + (direction[0] / direction[2]).powi(2) + (direction[1] / direction[2]).powi(2)).sqrt(),
		];

        let mut current_block = [
            start_position[0].floor() as i32,
            start_position[1].floor() as i32,
            start_position[2].floor() as i32,
        ];

        // check if starting point is already in block
        if check_coordinate_occupied(current_block) {
            return (true, vec![current_block]);
        }

        let mut encountered_blocks = vec![current_block];

		let mut travelled_ray: [f32; 3] = Default::default();
		let mut step: [i32; 3] = Default::default();

		// Establish Starting Conditions
		if direction[0] < 0.0
		{
			step[0] = -1;
			travelled_ray[0] = (start_position[0] - current_block[0] as f32) * unit_step_size[0];
		}
		else
		{
			step[0] = 1;
			travelled_ray[0] = ((current_block[0] + 1) as f32 - start_position[0]) * unit_step_size[0];
		}

		if direction[1] < 0.0
		{
			step[1] = -1;
			travelled_ray[1] = (start_position[1] - current_block[1] as f32) * unit_step_size[1];
		}
		else
		{
			step[1] = 1;
			travelled_ray[1] = ((current_block[1] + 1) as f32 - start_position[1]) * unit_step_size[1];
		}

        if direction[2] < 0.0
		{
			step[2] = -1;
			travelled_ray[2] = (start_position[2] - current_block[2] as f32) * unit_step_size[2];
		}
		else
		{
			step[2] = 1;
			travelled_ray[2] = ((current_block[2] + 1) as f32 - start_position[2]) * unit_step_size[2];
		}

		// Perform "Walk" until collision or range check
		
		let mut total_distance = 0.0;

		while total_distance < max_distance
		{
			// Walk along shortest path
			if travelled_ray[0] < travelled_ray[1] && travelled_ray[0] < travelled_ray[2]
			{
				current_block[0] += step[0];
				total_distance = travelled_ray[0];
				travelled_ray[0] += unit_step_size[0];
			}
			else if travelled_ray[1] < travelled_ray[2]
			{
				current_block[1] += step[1];
				total_distance = travelled_ray[1];
				travelled_ray[1] += unit_step_size[1];
			}
            else {
                current_block[2] += step[2];
                total_distance = travelled_ray[2];
                travelled_ray[2] += unit_step_size[2];
            }

            encountered_blocks.push(current_block);

			// Test tile at new test point
            if check_coordinate_occupied(current_block) {
                return (true, encountered_blocks);
            }
        }

        (false, encountered_blocks)
}