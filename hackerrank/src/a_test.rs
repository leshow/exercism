use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

struct Foo<F> {
    f: F,
}

impl<F: Future> Future for Foo<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().f).poll(cx) }
        // or unsafe { self.map_unchecked_mut(|s| &mut s.f) }.poll(cx);
    }
}

async fn run() {
    let f = Foo { f: async {} };
    f.await;
}
