use std::{error::Error, future::Future, marker::PhantomData, mem};

use async_channel::unbounded;
use tracing::{debug, info};

pub struct DownloadPool<'a, T, U, Fut>
where
    T: Send + 'a,
    U: Send + 'a,
    Fut: Future<Output = Result<U, Box<dyn Error>>> + Send + 'a,
{
    to_process: Vec<T>,
    worker_count: usize,
    task: fn(T) -> Fut,
    _marker: PhantomData<&'a ()>,
}

impl<'a, T, U, Fut> DownloadPool<'a, T, U, Fut>
where
    T: Send + 'a + 'static,
    U: Send + 'a,
    Fut: Future<Output = Result<U, Box<dyn Error>>> + Send + 'a + 'static,
{
    pub fn new(to_process: Vec<T>, worker_count: usize, task: fn(T) -> Fut) -> Self {
        Self {
            to_process,
            worker_count,
            task,
            _marker: PhantomData,
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn complete_tasks(&mut self) {
        info!("processing tasks");
        let (send_channel, recv_channel) = unbounded::<T>();

        let mut handles = Vec::with_capacity(self.worker_count);

        for _ in 0..self.worker_count {
            let recv_channel = recv_channel.clone();
            let t = self.task;

            let handle = tokio::spawn(async move {
                debug!("starting worker");
                while let Ok(task) = recv_channel.recv().await {
                    t(task).await;
                }
                debug!("worker finished");
            });

            handles.push(handle);
        }

        let tasks = mem::take(&mut self.to_process);
        for task in tasks.into_iter() {
            info!("sending task");
            send_channel.send(task).await.unwrap();
        }

        send_channel.close();
        futures::future::join_all(handles).await;
    }
}
