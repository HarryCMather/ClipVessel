use anyhow::anyhow;
use tauri::Emitter;

pub struct VideoProcessorService {
    is_running: bool
}

impl VideoProcessorService {
    pub(crate) fn new() -> VideoProcessorService {
        Self {
            is_running: false
        }
    }

    pub fn get_is_running(&self) -> bool {
        self.is_running
    }
    
    pub fn set_is_running(&mut self, app_handle: &tauri::AppHandle, is_running: bool) -> anyhow::Result<()> {
        const EVENT_NAME: &str = "job_running_state_changed";

        if self.is_running != is_running {
            self.is_running = is_running;
            app_handle.emit(EVENT_NAME, is_running)
                      .map_err(|err| anyhow!("Unable to update the {} event: {}", EVENT_NAME, err))?
        }
        
        Ok(())
    }
}
