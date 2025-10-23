use anyhow::Result;
use ethers::providers::{Provider, Http, Middleware};
use hyper::{
    service::{make_service_fn, service_fn},
        Body, Request, Response, Server,
        };
        use prometheus::{Encoder, TextEncoder};
        use std::sync::Arc;
        use tokio::time::{interval, Duration};
        use tracing::{info, error};

        mod blockchain;
        mod fork_detector;
        mod metrics;

        use blockchain::BlockchainMonitor;
        use metrics::Metrics;

        #[tokio::main]
        async fn main() -> Result<()> {
            tracing_subscriber::fmt::init();

                let rpc_url = std::env::var("RPC_URL")
                        .unwrap_or_else(|_| "http://mordor-node:8545".to_string());
                            
                                let poll_interval = std::env::var("POLL_INTERVAL_SECS")
                                        .unwrap_or_else(|_| "5".to_string())
                                                .parse::<u64>()?;

                                                    info!("Starting Mordor Fork Monitor");
                                                        info!("RPC URL: {}", rpc_url);
                                                            info!("Poll interval: {}s", poll_interval);

                                                                let provider = Provider::<Http>::try_from(&rpc_url)?;
                                                                    let metrics = Arc::new(Metrics::new());
                                                                        let monitor = Arc::new(BlockchainMonitor::new(provider, metrics.clone()));

                                                                            // Start monitoring loop
                                                                                let monitor_clone = monitor.clone();
                                                                                    tokio::spawn(async move {
                                                                                            let mut interval = interval(Duration::from_secs(poll_interval));
                                                                                                    loop {
                                                                                                                interval.tick().await;
                                                                                                                            if let Err(e) = monitor_clone.poll().await {
                                                                                                                                            error!("Monitoring error: {}", e);
                                                                                                                                                        }
                                                                                                                                                                }
                                                                                                                                                                    });

                                                                                                                                                                        // Start metrics HTTP server
                                                                                                                                                                            let metrics_clone = metrics.clone();
                                                                                                                                                                                let make_svc = make_service_fn(move |_| {
                                                                                                                                                                                        let metrics = metrics_clone.clone();
                                                                                                                                                                                                async move {
                                                                                                                                                                                                            Ok::<_, hyper::Error>(service_fn(move |req| {
                                                                                                                                                                                                                            serve_metrics(req, metrics.clone())
                                                                                                                                                                                                                                        }))
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                    });

                                                                                                                                                                                                                                                        let addr = ([0, 0, 0, 0], 9090).into();
                                                                                                                                                                                                                                                            let server = Server::bind(&addr).serve(make_svc);
                                                                                                                                                                                                                                                                
                                                                                                                                                                                                                                                                    info!("Metrics server listening on http://{}", addr);
                                                                                                                                                                                                                                                                        server.await?;

                                                                                                                                                                                                                                                                            Ok(())
                                                                                                                                                                                                                                                                            }

                                                                                                                                                                                                                                                                            async fn serve_metrics(
                                                                                                                                                                                                                                                                                req: Request<Body>,
                                                                                                                                                                                                                                                                                    metrics: Arc<Metrics>,
                                                                                                                                                                                                                                                                                    ) -> Result<Response<Body>, hyper::Error> {
                                                                                                                                                                                                                                                                                        if req.uri().path() == "/metrics" {
                                                                                                                                                                                                                                                                                                let encoder = TextEncoder::new();
                                                                                                                                                                                                                                                                                                        let metric_families = metrics.registry.gather();
                                                                                                                                                                                                                                                                                                                let mut buffer = vec![];
                                                                                                                                                                                                                                                                                                                        encoder.encode(&metric_families, &mut buffer).unwrap();
                                                                                                                                                                                                                                                                                                                                
                                                                                                                                                                                                                                                                                                                                        Ok(Response::new(Body::from(buffer)))
                                                                                                                                                                                                                                                                                                                                            } else if req.uri().path() == "/health" {
                                                                                                                                                                                                                                                                                                                                                    Ok(Response::new(Body::from("OK")))
                                                                                                                                                                                                                                                                                                                                                        } else {
                                                                                                                                                                                                                                                                                                                                                                Ok(Response::builder()
                                                                                                                                                                                                                                                                                                                                                                            .status(404)
                                                                                                                                                                                                                                                                                                                                                                                        .body(Body::from("Not Found"))
                                                                                                                                                                                                                                                                                                                                                                                                    .unwrap())
                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                        