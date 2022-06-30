use log::info;
use log_derive::logfn;
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver};

#[derive(Debug)]
pub struct Timer {
    rx: Receiver<()>,
    timeout: Duration,
}

// Timer fires after the specified duration. The timer can be renewed.
impl Timer {
    pub fn new(timeout: Duration) -> Timer {
        let (tx, rx) = channel(1);
        tokio::task::spawn(async move {
            tokio::time::sleep(timeout).await;
            info!("timeout in {:?}ms!", timeout);
            let _ = tx.send(());
        });

        Timer {
            timeout: timeout,
            rx: rx,
        }
    }

    pub fn renew(&mut self) {
        let (tx, rx) = channel(1);
        let timeout = self.timeout;
        tokio::task::spawn(async move {
            tokio::time::sleep(timeout).await;
            info!("timeout in {:?}ms!", timeout);
            let _ = tx.send(());
        });

        self.rx = rx;
    }

    pub fn get_rx(&mut self) -> &mut Receiver<()> {
        &mut self.rx
    }
}
