use log::{debug, info, warn};
use proxy_wasm::{
    traits::{Context, HttpContext, RootContext},
    types::{self, LogLevel, Status},
};
use serde::Deserialize;
use serde_json::{Map, Value};
use std::{cell::RefCell, collections::HashMap, error::Error, time::Duration};
use types::Action;

const POWERED_BY: &str = "header-augmenting-filter";
const CACHE_KEY: &str = "cache";
const INITIALISATION_TICK: Duration = Duration::from_secs(3);

#[derive(Deserialize, Debug)]
#[serde(default)]
struct FilterConfig {
    /// The Envoy cluster name housing a HTTP service that will provide headers
    /// to add to requests.
    header_providing_service_cluster: String,

    /// The path to call on the HTTP service providing headers.
    header_providing_service_path: String,

    /// The authority to set when calling the HTTP service providing headers.
    header_providing_service_authority: String,

    /// The length of time to keep headers cached.
    #[serde(with = "serde_humanize_rs")]
    header_cache_expiry: Duration,
}

impl Default for FilterConfig {
    fn default() -> Self {
        FilterConfig {
            header_providing_service_cluster: "sidecar".to_owned(),
            header_providing_service_path: "/headers".to_owned(),
            header_providing_service_authority: "sidecar".to_owned(),
            header_cache_expiry: Duration::from_secs(360),
        }
    }
}

thread_local! {
    static CONFIGS: RefCell<HashMap<u32, FilterConfig>> = RefCell::new(HashMap::new())
}

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|context_id| -> Box<dyn RootContext> {
        CONFIGS.with(|configs| {
            configs
                .borrow_mut()
                .insert(context_id, FilterConfig::default());
        });

        Box::new(RootHandler { context_id })
    });
    proxy_wasm::set_http_context(|_context_id, _root_context_id| -> Box<dyn HttpContext> {
        Box::new(HttpHandler {})
    })
}

struct RootHandler {
    context_id: u32,
}

impl RootContext for RootHandler {
    fn on_configure(&mut self, _config_size: usize) -> bool {
        let configuration: Vec<u8> = match self.get_configuration() {
            Some(c) => c,
            None => {
                warn!("mandatory configuration missing");
                return false;
            }
        };

        match serde_json::from_slice::<FilterConfig>(configuration.as_ref()) {
            Ok(config) => {
                info!("configuring: {:?}", config);
                CONFIGS.with(|configs| configs.borrow_mut().insert(self.context_id, config));

                // Initialisation tick.
                // TODO: Handle all these unwraps.
                self.set_tick_period(INITIALISATION_TICK);
                self.set_shared_data(CACHE_KEY, None, None).unwrap();

                true
            }
            Err(e) => {
                warn!("error parsing configuration: {:?}", e);
                false
            }
        }
    }

    fn on_tick(&mut self) {
        CONFIGS.with(|configs| {
            if let Some(config) = configs.borrow().get(&self.context_id) {
                // We could be in the initialisation tick here so update to
                // the configured expiry. This will be reset upon failures.
                self.set_tick_period(config.header_cache_expiry);

                // Log the current state.
                let (cache, _) = self.get_shared_data(CACHE_KEY);
                match cache {
                    None => debug!("initialising cached headers"),
                    Some(_) => debug!("refreshing cached headers"),
                }

                match self.dispatch_http_call(
                    &config.header_providing_service_cluster,
                    vec![
                        (":method", "GET"),
                        (":path", &config.header_providing_service_path),
                        (":authority", &config.header_providing_service_authority),
                    ],
                    None,
                    vec![],
                    Duration::from_secs(5),
                ) {
                    Err(e) => {
                        // Something went wrong instantly. Reset to an initialisation
                        // tick for a retry.
                        self.set_tick_period(INITIALISATION_TICK);
                        warn!("failed calling header providing service: {:?}", e)
                    }
                    Ok(_) => (),
                }
            }
        })
    }
}

impl Context for RootHandler {
    fn on_http_call_response(
        &mut self,
        _token_id: u32,
        _num_headers: usize,
        body_size: usize,
        _num_trailers: usize,
    ) {
        if let Some(body) = self.get_http_call_response_body(0, body_size) {
            match self.set_shared_data(CACHE_KEY, Some(&body), None) {
                Ok(()) | Err(Status::Ok) => debug!(
                    "refreshed header cache with: {}",
                    String::from_utf8(body.clone()).unwrap()
                ),
                Err(e) => {
                    self.set_tick_period(INITIALISATION_TICK);
                    warn!("failed header cache refresh: {:?}", e)
                }
            }
        }
    }
}

struct HttpHandler {}

impl HttpContext for HttpHandler {
    fn on_http_request_headers(&mut self, _num_headers: usize) -> Action {
        let (cache, _) = self.get_shared_data(CACHE_KEY);
        match cache {
            Some(data) => {
                debug!(
                    "using existing header cache: {}",
                    String::from_utf8(data.clone()).unwrap()
                );

                match self.parse_headers(&data) {
                    Ok(headers) => {
                        for (name, value) in headers {
                            self.set_http_request_header(&name, value.as_str())
                        }
                    }
                    Err(e) => {
                        warn!("no usable headers cached: {:?}", e);
                    }
                }

                Action::Continue
            }
            None => {
                warn!("filter not initialised");
                self.send_http_response(
                    500,
                    vec![("Powered-By", POWERED_BY)],
                    Some(b"Filter not initialised"),
                );

                Action::Pause
            }
        }
    }
}

impl Context for HttpHandler {}

impl HttpHandler {
    fn parse_headers(&self, res: &[u8]) -> Result<Map<String, Value>, Box<dyn Error>> {
        Ok(serde_json::from_slice::<Value>(&res)?
            .as_object()
            .unwrap()
            .clone())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn todo() {
        assert_eq!(2 + 2, 4);
    }
}
