//! HuggingFace model downloader that works behind corporate proxies.
//!
//! `fastembed`'s built-in downloader uses `hf-hub` which in turn uses `ureq` with
//! `webpki-roots`/`webpki-root-certs` (the bundled Mozilla trust store), so any
//! TLS-intercepting proxy with a private CA fails with `UnknownIssuer`.
//!
//! This module bypasses that by using `reqwest` configured to:
//! * pick up `http_proxy` / `https_proxy` / `no_proxy` from the environment
//!   (reqwest does this automatically),
//! * trust the OS native root store via `rustls-tls-native-roots`,
//! * additionally trust any PEM bundle pointed to by common env vars
//!   (`SSL_CERT_FILE`, `NODE_EXTRA_CA_CERTS`, `REQUESTS_CA_BUNDLE`,
//!   `CURL_CA_BUNDLE`, `PROJECT_RAG_CA_BUNDLE`).
//!
//! Files are cached on disk under `cache_dir/<repo>/` so a successful download
//! is reused across restarts. The cache layout is intentionally simple and
//! independent of fastembed's hf-hub layout.

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

const HF_BASE_URL_ENV: &str = "HF_ENDPOINT";
const HF_DEFAULT_BASE_URL: &str = "https://huggingface.co";
const CA_BUNDLE_ENV_VARS: &[&str] = &[
    "PROJECT_RAG_CA_BUNDLE",
    "SSL_CERT_FILE",
    "NODE_EXTRA_CA_CERTS",
    "REQUESTS_CA_BUNDLE",
    "CURL_CA_BUNDLE",
];

pub struct ModelDownloader {
    client: reqwest::blocking::Client,
    cache_dir: PathBuf,
    base_url: String,
}

impl ModelDownloader {
    pub fn new(cache_dir: PathBuf) -> Result<Self> {
        let mut builder = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(600))
            .connect_timeout(Duration::from_secs(30))
            .user_agent(concat!("project-rag/", env!("CARGO_PKG_VERSION")))
            // rustls-tls-native-roots loads the OS trust store; we keep it on as a
            // baseline and add corporate CAs from env vars on top.
            .tls_built_in_root_certs(true);

        let mut loaded_from = Vec::new();
        let mut already_seen = std::collections::HashSet::new();
        for var in CA_BUNDLE_ENV_VARS {
            if let Ok(path_str) = std::env::var(var) {
                if path_str.trim().is_empty() || !already_seen.insert(path_str.clone()) {
                    continue;
                }
                let path = PathBuf::from(&path_str);
                match load_extra_certs(&path) {
                    Ok(certs) if !certs.is_empty() => {
                        let n = certs.len();
                        for cert in certs {
                            builder = builder.add_root_certificate(cert);
                        }
                        loaded_from.push(format!("{}={} ({} cert(s))", var, path_str, n));
                    }
                    Ok(_) => tracing::warn!(
                        "{}={} contained no parseable certificates",
                        var,
                        path_str
                    ),
                    Err(e) => tracing::warn!(
                        "Failed to load CA bundle from {}={}: {:#}",
                        var,
                        path_str,
                        e
                    ),
                }
            }
        }
        if !loaded_from.is_empty() {
            tracing::info!("Trusting extra CA certs: {}", loaded_from.join(", "));
        }

        let client = builder
            .build()
            .context("Failed to build HTTPS client for model downloads")?;

        fs::create_dir_all(&cache_dir).with_context(|| {
            format!(
                "Failed to create model cache directory {}",
                cache_dir.display()
            )
        })?;

        let base_url = std::env::var(HF_BASE_URL_ENV)
            .ok()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| HF_DEFAULT_BASE_URL.to_string());

        Ok(Self {
            client,
            cache_dir,
            base_url,
        })
    }

    /// Download `filename` from `repo` (e.g. `Qdrant/all-MiniLM-L6-v2-onnx`)
    /// and return its bytes. The file is cached on disk under
    /// `<cache_dir>/<repo>/<filename>` so subsequent calls are offline.
    pub fn fetch(&self, repo: &str, filename: &str) -> Result<Vec<u8>> {
        let local_path = self.cache_dir.join(repo).join(filename);

        if let Ok(bytes) = fs::read(&local_path) {
            if !bytes.is_empty() {
                tracing::debug!(
                    "Using cached {} ({} bytes) from {}",
                    filename,
                    bytes.len(),
                    local_path.display()
                );
                return Ok(bytes);
            }
        }

        let url = format!(
            "{}/{}/resolve/main/{}",
            self.base_url.trim_end_matches('/'),
            repo,
            filename
        );
        tracing::info!("Downloading {} -> {}", url, local_path.display());

        let response = self
            .client
            .get(&url)
            .send()
            .with_context(|| format!("HTTP request failed for {}", url))?;

        let status = response.status();
        if !status.is_success() {
            anyhow::bail!("Unexpected status {} when fetching {}", status, url);
        }

        let bytes = response
            .bytes()
            .with_context(|| format!("Failed to read response body from {}", url))?
            .to_vec();

        if let Some(parent) = local_path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create cache subdir {}", parent.display())
            })?;
        }
        let tmp_path = local_path.with_extension("part");
        fs::write(&tmp_path, &bytes)
            .with_context(|| format!("Failed to write {}", tmp_path.display()))?;
        fs::rename(&tmp_path, &local_path)
            .with_context(|| format!("Failed to finalize {}", local_path.display()))?;

        Ok(bytes)
    }
}

fn load_extra_certs(path: &Path) -> Result<Vec<reqwest::Certificate>> {
    let pem = fs::read(path).with_context(|| format!("read {}", path.display()))?;

    if let Ok(certs) = reqwest::Certificate::from_pem_bundle(&pem) {
        if !certs.is_empty() {
            return Ok(certs);
        }
    }
    let cert = reqwest::Certificate::from_pem(&pem)
        .with_context(|| format!("parse {} as PEM", path.display()))?;
    Ok(vec![cert])
}

/// Default cache directory for downloaded model files.
///
/// Honors `PROJECT_RAG_MODEL_CACHE` and `FASTEMBED_CACHE_DIR` (in that order),
/// otherwise falls back to `<user-cache>/project-rag/models` and finally to
/// `./.fastembed_cache/manual` if no user cache dir is available.
pub fn default_cache_dir() -> PathBuf {
    if let Ok(p) = std::env::var("PROJECT_RAG_MODEL_CACHE") {
        if !p.trim().is_empty() {
            return PathBuf::from(p);
        }
    }
    if let Ok(p) = std::env::var("FASTEMBED_CACHE_DIR") {
        if !p.trim().is_empty() {
            return PathBuf::from(p).join("manual");
        }
    }
    if let Some(base) = dirs::cache_dir() {
        return base.join("project-rag").join("models");
    }
    PathBuf::from(".fastembed_cache").join("manual")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_cache_dir_respects_env_override() {
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().to_string_lossy().to_string();
        // SAFETY: the test owns the env exclusively for this scope.
        unsafe {
            std::env::set_var("PROJECT_RAG_MODEL_CACHE", &target);
        }
        assert_eq!(default_cache_dir(), PathBuf::from(&target));
        unsafe {
            std::env::remove_var("PROJECT_RAG_MODEL_CACHE");
        }
    }

    #[test]
    fn downloader_creates_cache_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let cache = tmp.path().join("nested").join("cache");
        let _ = ModelDownloader::new(cache.clone()).expect("build downloader");
        assert!(cache.is_dir());
    }
}
