window.subscribeToJobRunningStateChanges = function(dotnetRef) {
    window.__TAURI__.event.listen("job_running_state_changed", event => {
       dotnetRef.invokeMethodAsync("OnJobRunningStateChanged", event.payload);
    });
}
