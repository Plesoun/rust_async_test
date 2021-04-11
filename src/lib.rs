use futures::future::Future;
use futures::task::Context;
use futures::task::Poll;

use std::pin::Pin;

pub struct SimpleFuture {
	n:i32,
}

impl Future for SimpleFuture {
	type Output = i32;
	fn poll(self: Pin<&mut Self>, _cx:&mut Context) -> Poll<Self::Output> {
		Poll::Ready(self.n)
	}
}

pub async fn simpleasync(p:i32) -> i32 {
	p + 10
}


#[cfg(test)]
mod tests {
	use super::*;
	use futures::executor::block_on;
	use futures::future::FutureExt;
	use futures::channel::oneshot;
	#[test]
	fn test_future() {

		let f = simpleasync(5);

		//let f = SimpleFuture{n:10};

		let (channel_sender, channel_reciever) = oneshot::channel();


		block_on(f.map(move |n| channel_sender.send(n+5)));

		let result = block_on(channel_reciever).unwrap();

		assert_eq!(result, 20);
		//Pin::new(f).poll()
	}

	#[test]
	fn test_async_send() {
		let (channel_sender, channel_reciever) = oneshot::channel();
		block_on(async move {
			let v = simpleasync(40).await;
			channel_sender.send(v)
		}).unwrap();

		let fin_res = block_on(async move {
			let result = channel_reciever.await.unwrap();
			result + 10
		});

		assert_eq!(fin_res, 60);
	}
}
