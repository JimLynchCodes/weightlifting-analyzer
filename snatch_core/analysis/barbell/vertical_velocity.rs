fn vertical_velocity(frames: &[PoseFrame]) -> Vec<f32> {
    let mut velocities = Vec::new();

    for window in frames.windows(2) {
        let f1 = &window[0];
        let f2 = &window[1];

        if let (Some(b1), Some(b2)) = (&f1.barbell, &f2.barbell) {
            let dt = f2.timestamp - f1.timestamp;

            if dt > 0.0 {
                let vy = (b2.center_y - b1.center_y) / dt;
                velocities.push(-vy); // upward positive
            } else {
                velocities.push(0.0);
            }
        } else {
            velocities.push(0.0);
        }
    }

    velocities
}
