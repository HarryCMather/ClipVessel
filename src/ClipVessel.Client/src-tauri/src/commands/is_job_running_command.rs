use std::sync::{Arc, Mutex, MutexGuard};
use tauri::State;
use crate::services::video_processor_service::VideoProcessorService;

#[tauri::command]
pub fn is_job_running(video_processor_service: State<'_, Arc<Mutex<VideoProcessorService>>>) -> bool {
    let video_processor_service: MutexGuard<VideoProcessorService> = video_processor_service.lock().unwrap();
    video_processor_service.get_is_running()
}
