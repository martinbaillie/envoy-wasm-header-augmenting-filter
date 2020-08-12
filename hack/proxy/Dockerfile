FROM istio/proxyv2:1.5.8
ENTRYPOINT /usr/local/bin/envoy \
    -c /etc/envoy.yaml \
    -l debug \ 
    --component-log-level conn_handler:info,router:info,connection:info,pool:info,client:info,upstream:info,main:info \
    --service-cluster proxy 
