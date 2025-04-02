mod confg;
mod llm;
mod metric;

use async_trait::async_trait;
use clap::Parser;
use dotenv::dotenv;
use log::info;
use pingora_core::listeners::tls::TlsSettings;
use pingora_core::modules::http::HttpModules;
use pingora_core::server::{configuration::Opt, Server};
use pingora_core::upstreams::peer::HttpPeer;
use pingora_error::Result;
use pingora_http::{Method, ResponseHeader};
use pingora_load_balancing::{selection::RoundRobin, LoadBalancer};
use pingora_proxy::{ProxyHttp, Session};
use std::sync::Arc;

// TODO:
// - LB: Load Balancing
// - LLM: LLM
// - DConf: Dynamic configuration or Domain configuration for ConfMgr
// - Transform: Transform the request and response
// - Router: Route the request to the upstream
// - Cache: Cache the response
// - RateLimit: Rate limit the request
// - Auth: Authentication
// - WAF: Web Application Firewall
// - Listen: 不重启，动态更新

pub struct Cygress {
    metric: metric::Metric,
    conf: confg::Cfg,
}

#[async_trait]
impl ProxyHttp for Cygress {
    type CTX = (Arc<LoadBalancer<RoundRobin>>);
    fn new_ctx(&self) -> Self::CTX {
        let upstreams = LoadBalancer::try_from_iter(["dashscope.aliyuncs.com:443"]).unwrap();
        Arc::new(upstreams)
    }

    // This function is only called once when the server starts
    fn init_downstream_modules(&self, modules: &mut HttpModules) {
        modules.add_module(Box::new(llm::LLM {}))
    }

    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
        Ok(false)
    }

    async fn upstream_peer(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let addr = ctx
            .select(b"", 256) // hash doesn't matter
            .unwrap();

        info!("connecting to {addr:?}");
        let peer = Box::new(HttpPeer::new(
            addr,
            true,
            "dashscope.aliyuncs.com".to_string(),
        ));
        Ok(peer)
    }

    async fn response_filter(
        &self,
        _session: &mut Session,
        upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()>
    where
        Self::CTX: Send + Sync,
    {
        Ok(())
    }

    async fn logging(
        &self,
        session: &mut Session,
        _e: Option<&pingora_core::Error>,
        ctx: &mut Self::CTX,
    ) {
        let response_code = session
            .response_written()
            .map_or(0, |resp| resp.status.as_u16());

        self.metric.req_metric.inc();
    }

    async fn upstream_request_filter(
        &self,
        session: &mut Session,
        upstream_request: &mut pingora_http::RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        session.req_header_mut().set_method(Method::POST);
        upstream_request
            .insert_header("Host", "dashscope.aliyuncs.com")
            .unwrap();
        Ok(())
    }
}

fn main() {
    // load .env file
    dotenv().ok();
    log4rs::init_file("cygress/log4rs.yaml", Default::default()).unwrap();

    let opt = Opt::parse();
    let mut s = Server::new(Some(opt)).unwrap();
    s.bootstrap();

    // Load static configuration
    let _sconf = confg::Cfg::load_from_env();
    // TODO: handle sconf

    // 如何动态去更新dconf？

    let mut cyg = pingora_proxy::http_proxy_service(
        &s.configuration,
        Cygress {
            metric: metric::Metric::new(),
            conf: _sconf,
        },
    );

    let cert_path = format!("{}/tests/keys/server.crt", env!("CARGO_MANIFEST_DIR"));
    let key_path = format!("{}/tests/keys/key.pem", env!("CARGO_MANIFEST_DIR"));

    let mut tls_settings = TlsSettings::intermediate(&cert_path, &key_path).unwrap();
    tls_settings.enable_h2();
    cyg.add_tls_with_settings("0.0.0.0:6191", None, tls_settings);
    cyg.add_tcp("0.0.0.0:6190");
    s.add_service(cyg);

    // 127.0.0.1:343" is just a bad server
    // let mut upstreams =
    //     LoadBalancer::try_from_iter(["1.1.1.1:443", "1.0.0.1:443", "127.0.0.1:343"]).unwrap();

    // We add health check in the background so that the bad server is never selected.
    // let hc = health_check::TcpHealthCheck::new();
    // upstreams.set_health_check(hc);
    // upstreams.health_check_frequency = Some(Duration::from_secs(1));

    // let background = background_service("health check", upstreams);

    // let upstreams = background.task();

    let mut prometheus_service_http =
        pingora_core::services::listening::Service::prometheus_http_service();
    prometheus_service_http.add_tcp("127.0.0.1:6192");
    s.add_service(prometheus_service_http);
    // my_server.add_service(background);

    s.run_forever();
}
