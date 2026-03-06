use crate::types::*;

const DEFAULT_BASE_URL: &str = "https://rfidfyi.com/api";

/// Async client for the RFIDFYI API.
pub struct Client {
    base_url: String,
    http: reqwest::Client,
}

impl Client {
    /// Creates a new client with the default base URL.
    pub fn new() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            http: reqwest::Client::new(),
        }
    }

    /// Creates a new client with a custom base URL.
    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http: reqwest::Client::new(),
        }
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, RfidFyiError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(RfidFyiError::Api {
                status: resp.status().as_u16(),
                body: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(resp.json().await?)
    }

    /// Search across RFID tags, readers, and glossary terms.
    pub async fn search(&self, query: &str) -> Result<SearchResult, RfidFyiError> {
        let encoded = urlencoding(query);
        self.get(&format!("/search/?q={}", encoded)).await
    }

    /// Get details for an RFID tag by slug.
    pub async fn tag(&self, slug: &str) -> Result<TagDetail, RfidFyiError> {
        self.get(&format!("/tag/{}/", slug)).await
    }

    /// Get details for an RFID reader by slug.
    pub async fn reader(&self, slug: &str) -> Result<ReaderDetail, RfidFyiError> {
        self.get(&format!("/reader/{}/", slug)).await
    }

    /// Get details for an RFID family by slug.
    pub async fn family(&self, slug: &str) -> Result<FamilyDetail, RfidFyiError> {
        self.get(&format!("/family/{}/", slug)).await
    }

    /// Get details for an RFID frequency band by slug.
    pub async fn frequency(&self, slug: &str) -> Result<FrequencyDetail, RfidFyiError> {
        self.get(&format!("/frequency/{}/", slug)).await
    }

    /// Get details for an RFID standard by slug.
    pub async fn standard(&self, slug: &str) -> Result<StandardDetail, RfidFyiError> {
        self.get(&format!("/standard/{}/", slug)).await
    }

    /// Get details for an EPC standard by slug.
    pub async fn epc(&self, slug: &str) -> Result<EpcDetail, RfidFyiError> {
        self.get(&format!("/epc/{}/", slug)).await
    }

    /// Get details for an RFID use case by slug.
    pub async fn use_case(&self, slug: &str) -> Result<UseCaseDetail, RfidFyiError> {
        self.get(&format!("/use-case/{}/", slug)).await
    }

    /// Get a glossary term by slug.
    pub async fn glossary_term(&self, slug: &str) -> Result<GlossaryTerm, RfidFyiError> {
        self.get(&format!("/glossary/{}/", slug)).await
    }

    /// Compare two RFID tags.
    pub async fn compare(&self, slug_a: &str, slug_b: &str) -> Result<CompareResult, RfidFyiError> {
        self.get(&format!("/compare/?a={}&b={}", slug_a, slug_b)).await
    }

    /// Get a random RFID tag.
    pub async fn random(&self) -> Result<TagDetail, RfidFyiError> {
        self.get("/random/").await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "+".to_string(),
            _ => format!("%{:02X}", c as u32),
        })
        .collect()
}
