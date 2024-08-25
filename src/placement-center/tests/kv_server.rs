#[cfg(test)]
mod tests {
    use std::net::TcpStream;

    use axum::http::request;
    use protocol::kv::{
        kv_service_client::KvServiceClient, DeleteRequest, ExistsReply, ExistsRequest, GetRequest,
        SetRequest,
    };

    #[tokio::test]
    async fn kv_test() {
        let mut client = KvServiceClient::connect("http://127.0.0.1:8871")
            .await
            .unwrap();
        let key = "mq".to_string();
        let value = "robustmq".to_string();
        let request = tonic::Request::new(SetRequest {
            key: key.clone(),
            value: value.clone(),
        });

        let _ = client.set(request).await.unwrap();

        let request = tonic::Request::new(ExistsRequest { key: key.clone() });
        let exist_reply = client.exists(request).await.unwrap().into_inner();
        assert!(exist_reply.flag);

        let request = tonic::Request::new(GetRequest { key: key.clone() });
        let get_reply = client.get(request).await.unwrap().into_inner();
        assert_eq!(get_reply.value, value);

        let request = tonic::Request::new(DeleteRequest { key: key.clone() });
        let _ = client.delete(request).await.unwrap().into_inner();

        let request = tonic::Request::new(ExistsRequest { key: key.clone() });
        let exist_reply = client.exists(request).await.unwrap().into_inner();
        assert!(!exist_reply.flag);
    }
}
