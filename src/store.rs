use futures_util::StreamExt;
use k8s_openapi::serde::de::DeserializeOwned;
use kube::runtime::reflector::Store;
use kube::runtime::watcher::Config;
use kube::{Api, Client, Resource};
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;

pub struct StoreHandle {
    inner: Pin<Box<dyn Future<Output = anyhow::Result<()>> + 'static + Send>>,
}

impl StoreHandle {
    fn new<F>(future: F) -> Self
    where
        F: Future<Output = anyhow::Result<()>> + 'static + Send,
    {
        let inner = Box::pin(future);
        Self { inner }
    }
}

impl Future for StoreHandle {
    type Output = anyhow::Result<()>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let pinned = std::pin::Pin::new(&mut self.inner);
        pinned.poll(cx)
    }
}

pub fn cluster_store<K>(client: Client) -> anyhow::Result<(Store<K>, StoreHandle)>
where
    K: 'static + Resource<DynamicType = ()> + Clone + Debug + Send + DeserializeOwned + Sync,
{
    let api: Api<K> = Api::all(client.clone());
    let wc = Config::default();
    let (reader, writer) = kube::runtime::reflector::store();
    let mut reflector = Box::pin(kube::runtime::reflector(
        writer,
        kube::runtime::watcher(api, wc),
    ));

    let spawn_handle = tokio::spawn(async move {
        while let Some(_item) = reflector.next().await {
            // just keep pumping messages
        }
        Ok(())
    });
    let store_handle = StoreHandle::new(async move { spawn_handle.await? });
    Ok((reader, store_handle))
}
