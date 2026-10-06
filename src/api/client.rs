use crate::api::errors::ApiError;
use crate::apis::ContentType;
use crate::apis::configuration::ApiKey;
use crate::models;
use serde::de::Error;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use tokio::io::AsyncRead;
use tokio_util::codec::{BytesCodec, FramedRead};

pub struct ApiClient {
    base_path: String,
    client: reqwest::Client,
    api_key: ApiKey,
    user_agent: Option<String>,
}

impl ApiClient {
    pub fn base_path(&self) -> &str {
        &self.base_path
    }

    // -------------------------------------------------------------------------
    // Request helpers
    // -------------------------------------------------------------------------

    fn request(
        &self,
        method: reqwest::Method,
        uri: &str,
        headers: impl IntoIterator<Item = (String, String)>,
    ) -> reqwest::RequestBuilder {
        let mut request = self.client.request(method, uri);

        if let Some(user_agent) = &self.user_agent {
            request = request.header(reqwest::header::USER_AGENT, user_agent);
        }

        for (name, value) in headers {
            request = request.header(name, value);
        }

        request
    }

    async fn execute(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, ApiError> {
        let response = request.send().await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(ApiError::GetInfo(
                response
                    .error_for_status()
                    .expect_err("non-success status should produce an error"),
            ))
        }
    }

    async fn get<T: DeserializeOwned>(&self, uri: &str) -> Result<T, ApiError> {
        let response = self
            .execute(self.request(reqwest::Method::GET, uri, []))
            .await?;

        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("application/octet-stream");

        match ContentType::from(content_type) {
            ContentType::Json => Ok(serde_json::from_str(&response.text().await?)?),

            ContentType::Text => Err(ApiError::from(serde_json::Error::custom(
                "Received `text/plain` content type response",
            ))),

            ContentType::Unsupported(unknown_type) => Err(ApiError::from(
                serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response"
                )),
            )),
        }
    }

    async fn put(
        &self,
        uri: &str,
        headers: HashMap<String, String>,
    ) -> Result<(), ApiError> {
        self.execute(self.request(reqwest::Method::PUT, uri, headers))
            .await?;

        Ok(())
    }

    async fn delete(
        &self,
        uri: &str,
        headers: HashMap<String, String>,
    ) -> Result<(), ApiError> {
        self.execute(self.request(reqwest::Method::DELETE, uri, headers))
            .await?;

        Ok(())
    }

    async fn put_with_body<R>(
        &self,
        uri: &str,
        headers: HashMap<String, String>,
        body: R,
    ) -> Result<(), ApiError>
    where
        R: AsyncRead + Send + 'static,
    {
        let stream = FramedRead::new(body, BytesCodec::new());

        let request = self
            .request(reqwest::Method::PUT, uri, headers)
            .body(reqwest::Body::wrap_stream(stream));

        self.execute(request).await?;

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Get methods
    // -------------------------------------------------------------------------

    pub async fn get_api_version(&self) -> Result<models::version::Version, ApiError> {
        self.get(&format!("{}/api/version", self.base_path)).await
    }

    pub async fn get_camera(
        &self,
        id: &str,
    ) -> Result<models::camera_config::CameraConfig, ApiError> {
        let uri = format!(
            "{}/api/v1/cameras/{}",
            self.base_path,
            crate::apis::urlencode(id)
        );

        self.get(&uri).await
    }

    pub async fn get_file_metadata(
        &self,
        storage: &str,
        path: &str,
        _accept_language: Option<&str>,
        _accept: Option<&str>,
    ) -> Result<models::GetFileMetadata200Response, ApiError> {
        let uri = format!(
            "{}/api/v1/files/{}/{}",
            self.base_path,
            crate::apis::urlencode(storage),
            crate::apis::urlencode(path),
        );

        self.get(&uri).await
    }

    pub async fn get_job(&self) -> Result<models::Job, ApiError> {
        self.get(&format!("{}/api/v1/job", self.base_path)).await
    }

    pub async fn get_info(&self) -> Result<models::Info, ApiError> {
        self.get(&format!("{}/api/v1/info", self.base_path)).await
    }

    pub async fn get_status(
        &self,
    ) -> Result<models::GetPrinterStatus200Response, ApiError> {
        self.get(&format!("{}/api/v1/status", self.base_path)).await
    }

    pub async fn list_storage(
        &self,
        _accept_language: Option<&str>,
    ) -> Result<models::GetStorage200Response, ApiError> {
        self.get(&format!("{}/api/v1/storage", self.base_path)).await
    }

    pub async fn get_transfer(&self) -> Result<models::Transfer, ApiError> {
        self.get(&format!("{}/api/v1/transfer", self.base_path)).await
    }

    pub async fn list_cameras(&self) -> Result<Vec<models::Camera>, ApiError> {
        self.get(&format!("{}/api/v1/cameras", self.base_path)).await
    }

    // -------------------------------------------------------------------------
    // File operations
    // -------------------------------------------------------------------------

    pub async fn upload_file<R: AsyncRead + Send + 'static>(
        &self,
        storage: &str,
        path: &str,
        body: R,
        accept_language: Option<&str>,
        accept: Option<&str>,
        content_length: Option<i32>,
        content_type: Option<&str>,
        print_after_upload: Option<&str>,
        overwrite: Option<&str>,
    ) -> Result<(), ApiError> {
        let uri = format!(
            "{}/api/v1/files/{}/{}",
            self.base_path,
            crate::apis::urlencode(storage),
            crate::apis::urlencode(path),
        );

        let mut headers = HashMap::new();

        insert_header(&mut headers, "Accept-Language", accept_language);
        insert_header(&mut headers, "Accept", accept);
        insert_header(
            &mut headers,
            "Content-Length",
            content_length.map(|value| value.to_string()),
        );
        insert_header(&mut headers, "Content-Type", content_type);
        insert_header(&mut headers, "Print-After-Upload", print_after_upload);
        insert_header(&mut headers, "Overwrite", overwrite);

        self.put_with_body(&uri, headers, body).await
    }

    pub async fn pause_job(&self, id: i32) -> Result<(), ApiError> {
        self.put(
            &format!("{}/api/v1/job/{id}/pause", self.base_path),
            HashMap::new(),
        )
        .await
    }

    pub async fn stop_job(&self, id: i32) -> Result<(), ApiError> {
        self.delete(
            &format!("{}/api/v1/job/{id}", self.base_path),
            HashMap::new(),
        )
        .await
    }

    pub async fn resume_job(&self, id: i32) -> Result<(), ApiError> {
        self.put(
            &format!("{}/api/v1/job/{id}/resume", self.base_path),
            HashMap::new(),
        )
        .await
    }

    pub async fn delete_file(
        &self,
        storage: &str,
        path: &str,
        accept_language: Option<&str>,
        accept: Option<&str>,
        force: Option<&str>,
    ) -> Result<(), ApiError> {
        let uri = format!(
            "{}/api/v1/files/{}/{}",
            self.base_path,
            crate::apis::urlencode(storage),
            crate::apis::urlencode(path),
        );

        let mut headers = HashMap::new();

        insert_header(&mut headers, "Accept-Language", accept_language);
        insert_header(&mut headers, "Accept", accept);
        insert_header(&mut headers, "Force", force);

        self.delete(&uri, headers).await
    }

    pub async fn stop_transfer(&self, id: i32) -> Result<(), ApiError> {
        self.delete(&format!("{}/api/v1/transfer/{id}", self.base_path, id=id), HashMap::new()).await
    }
}

// -----------------------------------------------------------------------------
// Header helpers
// -----------------------------------------------------------------------------

fn insert_header(
    headers: &mut HashMap<String, String>,
    name: &str,
    value: Option<impl Into<String>>,
) {
    if let Some(value) = value {
        headers.insert(name.to_owned(), value.into());
    }
}