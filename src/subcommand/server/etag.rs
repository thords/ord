use super::*;

#[derive(Clone)]
pub(crate) struct EtagLayer<S> {
  inner: PhantomData<S>,
}

impl<S> EtagLayer<S> {
  pub(crate) fn new() -> Self {
    Self {
      inner: PhantomData::default(),
    }
  }
}
#[derive(Clone)]
struct Etag<S> {
  inner: S,
}

impl<S> Layer<S> for EtagLayer<S> {
  type Service = Etag<S>;

  fn layer(&self, inner: S) -> Self::Service {
    Etag { inner }
  }
}

struct ResponseFuture<F>;

impl<ReqBody, S> Service<Request<ReqBody>> for Etag<S>
where
  S: Service<Request<ReqBody>, Response = Response<Body>>,
{
  type Response = Response<Body>;
  type Error = S::Error;
  type Future = ResponseFuture<S::Future>;

  #[inline]
  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.inner.poll_ready(cx)
  }

  fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
    todo!()
  }
}
