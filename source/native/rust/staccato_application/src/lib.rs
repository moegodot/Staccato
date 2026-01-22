pub struct Application {
    event_loop: EventLoop,
    windows: HashMap<u32, Window>, // Key 是 WindowID
}

impl Application {
    pub fn run(&mut self) {
        let mut running = true;
        while running {
            self.event_loop.poll_events(|ev| {
                match ev {
                    StaccatoEvent::WindowClose { window_id } => {
                        // 如果 window_id 是主窗口，设置 running = false
                        running = false;
                    }
                    StaccatoEvent::WindowResized { window_id, width, height } => {
                        if let Some(win) = self.windows.get_mut(&window_id) {
                            // 这里可以触发 wgpu surface 的 resize
                            win.handle_resize(width, height);
                        }
                    }
                    _ => {}
                }

                // --- 关键点：将事件同步给 C# ---
                // NativeToManaged::dispatch_event(ev);
            });

            // 执行渲染流
            for win in self.windows.values_mut() {
                win.render();
            }
        }
    }
}
