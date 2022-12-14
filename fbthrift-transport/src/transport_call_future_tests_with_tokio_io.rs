use crate::configuration::AsyncTransportConfiguration;

use super::Call;

//
use std::io::Cursor;
fn block_on<T>(future: impl std::future::Future<Output = T>) -> T {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(future)
}
