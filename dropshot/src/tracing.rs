#[cfg(feature = "usdt-probes")]
pub use crate::dtrace::Dtrace;
use async_trait::async_trait;

pub use crate::{dtrace::RequestInfo, dtrace::ResponseInfo};

#[async_trait]
pub trait Tracing: std::fmt::Debug + Clone + Send + Sync + 'static {
    type Registration: Unpin;

    fn register(&self) -> Self::Registration;
    async fn request_start(&mut self, request_info: &RequestInfo);
    async fn request_done(&self, response_info: &ResponseInfo);
}

#[cfg(feature = "usdt-probes")]

mod dtrace {

    #[derive(Debug, Clone, Copy)]
    pub struct Dtrace;

    #[async_trait]
    impl Tracing for Dtrace {
        type Registration = crate::dtrace::ProbeRegistration;

        fn register(&self) -> Self::Registration {
            match usdt::register_probes() {
                Ok(()) => crate::dtrace::ProbeRegistration::Succeeded,
                Err(e) => crate::dtrace::ProbeRegistration::Failed(e),
            }
        }

        async fn request_start(&mut self, request_info: &RequestInfo) {
            crate::dtrace::probes::request__start!(|| request_info);
        }

        async fn request_done(&self, response_info: &ResponseInfo) {
            crate::dtrace::probes::request_done!(|| response_info);
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Noop;

#[async_trait]
impl Tracing for Noop {
    type Registration = ();

    fn register(&self) -> Self::Registration {
        ()
    }

    async fn request_start(&mut self, _: &RequestInfo) {}

    async fn request_done(&self, _: &ResponseInfo) {}
}
