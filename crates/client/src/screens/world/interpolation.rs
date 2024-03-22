use std::time::Duration;

use interpolation::lerp;

use super::entity::Character;

impl Character {
    /// use position buffer to set current position
    pub fn interpolate(&mut self, delta: Duration) {
        if let Some(target) = self.position_buffer.first().cloned() {
            if self.interpolation_time.as_millis() as f32 - delta.as_millis() as f32 <= 0. {
                self.just_finished_moving = true;
                self.position.x = target.x;
                self.position.y = target.y;
                self.position_buffer.remove(0);
                // check tile exit
            } else {
                self.just_finished_moving = false;
                self.just_started_moving = false;
            }
            self.interpolation_time = self
                .interpolation_time
                .checked_sub(delta)
                .unwrap_or(Duration::ZERO);
            let interpolation_progress = 1. - self.interpolation_time.as_millis() as f32 / 200.;
            let x = lerp(
                &(self.position.x as f32),
                &(target.x as f32),
                &interpolation_progress,
            );
            let y = lerp(
                &(self.position.y as f32),
                &(target.y as f32),
                &interpolation_progress,
            );
            self.render_position = (x * 32., y * 32.);

            if self.just_finished_moving && !self.position_buffer.is_empty() {
                self.interpolation_time = Duration::from_millis(200);
                if let Some(direction) = self.position.get_direction(&self.position_buffer[0]) {
                    self.change_direction(direction);
                }
                self.just_started_moving = true;
            }
        }
    }
}
