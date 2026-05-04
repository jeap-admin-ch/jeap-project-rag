use super::EmbeddingProvider;
use anyhow::{Context, Result};
use fastembed::{
    EmbeddingModel, InitOptions, InitOptionsUserDefined, TextEmbedding, TokenizerFiles,
    UserDefinedEmbeddingModel,
};
use std::fs;
use std::path::Path;
use std::sync::RwLock;

/// FastEmbed-based embedding provider using all-MiniLM-L6-v2
///
/// Uses RwLock for safe interior mutability since fastembed's embed() requires &mut self.
pub struct FastEmbedManager {
    model: RwLock<TextEmbedding>,
    dimension: usize,
    model_name: String,
}

impl FastEmbedManager {
    /// Create a new FastEmbedManager with the default model (all-MiniLM-L6-v2)
    pub fn new() -> Result<Self> {
        Self::with_model(EmbeddingModel::AllMiniLML6V2)
    }

    /// Create a new FastEmbedManager from a model name string
    pub fn from_model_name(model_name: &str) -> Result<Self> {
        let model = parse_model_name(model_name);
        Self::with_model_named(model, model_name.to_string())
    }

    /// Create a new FastEmbedManager with a specific model
    pub fn with_model(model: EmbeddingModel) -> Result<Self> {
        let name = canonical_name_for(&model);
        Self::with_model_named(model, name)
    }

    fn with_model_named(model: EmbeddingModel, name: String) -> Result<Self> {
        tracing::info!("Initializing FastEmbed model: {:?}", model);

        let dimension = dimension_for(&model);

        let mut options = InitOptions::default();
        options.model_name = model;
        options.show_download_progress = true;

        let embedding_model =
            TextEmbedding::try_new(options).context("Failed to initialize FastEmbed model")?;

        Ok(Self {
            model: RwLock::new(embedding_model),
            dimension,
            model_name: name,
        })
    }

    /// Create a new FastEmbedManager by loading model and tokenizer files from a
    /// local directory. This is fully offline and is intended for environments
    /// where fastembed's built-in HuggingFace download fails (e.g. behind a
    /// TLS-intercepting corporate proxy) or for containerized deployments that
    /// bake the model into the image.
    ///
    /// The directory must contain:
    /// - `model.onnx` (or `model_quantized.onnx` as a fallback)
    /// - `tokenizer.json`
    /// - `config.json`
    /// - `special_tokens_map.json`
    /// - `tokenizer_config.json`
    ///
    /// `model_name` is still required: it determines the embedding dimension
    /// (e.g. 384 for `all-MiniLM-L6-v2`, 768 for `BAAI/bge-base-en-v1.5`).
    pub fn from_local_path(model_path: &Path, model_name: &str) -> Result<Self> {
        tracing::info!(
            "Loading FastEmbed model '{}' from local path: {}",
            model_name,
            model_path.display()
        );

        if !model_path.is_dir() {
            anyhow::bail!(
                "Model directory does not exist or is not a directory: {}",
                model_path.display()
            );
        }

        let onnx_file = read_onnx_file(model_path)?;
        let tokenizer_files = TokenizerFiles {
            tokenizer_file: read_required(model_path, "tokenizer.json")?,
            config_file: read_required(model_path, "config.json")?,
            special_tokens_map_file: read_required(model_path, "special_tokens_map.json")?,
            tokenizer_config_file: read_required(model_path, "tokenizer_config.json")?,
        };

        let user_model = UserDefinedEmbeddingModel::new(onnx_file, tokenizer_files);
        let options = InitOptionsUserDefined::default();

        let embedding_model = TextEmbedding::try_new_from_user_defined(user_model, options)
            .context("Failed to initialize FastEmbed model from local files")?;

        let model_enum = parse_model_name(model_name);
        let dimension = dimension_for(&model_enum);

        Ok(Self {
            model: RwLock::new(embedding_model),
            dimension,
            model_name: model_name.to_string(),
        })
    }
}

fn parse_model_name(model_name: &str) -> EmbeddingModel {
    match model_name {
        "all-MiniLM-L6-v2" => EmbeddingModel::AllMiniLML6V2,
        "all-MiniLM-L12-v2" => EmbeddingModel::AllMiniLML12V2,
        "BAAI/bge-base-en-v1.5" => EmbeddingModel::BGEBaseENV15,
        "BAAI/bge-small-en-v1.5" => EmbeddingModel::BGESmallENV15,
        _ => {
            tracing::warn!(
                "Unknown model '{}', falling back to all-MiniLM-L6-v2",
                model_name
            );
            EmbeddingModel::AllMiniLML6V2
        }
    }
}

fn canonical_name_for(model: &EmbeddingModel) -> String {
    match model {
        EmbeddingModel::AllMiniLML6V2 => "all-MiniLM-L6-v2".to_string(),
        EmbeddingModel::AllMiniLML12V2 => "all-MiniLM-L12-v2".to_string(),
        EmbeddingModel::BGEBaseENV15 => "BAAI/bge-base-en-v1.5".to_string(),
        EmbeddingModel::BGESmallENV15 => "BAAI/bge-small-en-v1.5".to_string(),
        other => format!("{:?}", other),
    }
}

fn dimension_for(model: &EmbeddingModel) -> usize {
    match model {
        EmbeddingModel::AllMiniLML6V2 => 384,
        EmbeddingModel::AllMiniLML12V2 => 384,
        EmbeddingModel::BGEBaseENV15 => 768,
        EmbeddingModel::BGESmallENV15 => 384,
        _ => 384,
    }
}

fn read_required(dir: &Path, filename: &str) -> Result<Vec<u8>> {
    let path = dir.join(filename);
    fs::read(&path).with_context(|| {
        format!(
            "Failed to read required model file {} (expected in {})",
            filename,
            dir.display()
        )
    })
}

fn read_onnx_file(dir: &Path) -> Result<Vec<u8>> {
    for candidate in ["model.onnx", "model_quantized.onnx"] {
        let path = dir.join(candidate);
        if path.is_file() {
            return fs::read(&path)
                .with_context(|| format!("Failed to read ONNX model at {}", path.display()));
        }
    }
    anyhow::bail!(
        "No ONNX model file found in {}. Expected one of: model.onnx, model_quantized.onnx",
        dir.display()
    );
}

impl EmbeddingProvider for FastEmbedManager {
    fn embed_batch(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(vec![]);
        }

        tracing::debug!("Generating embeddings for {} texts", texts.len());

        // Acquire write lock safely. If the lock is poisoned (due to a panic while holding
        // the lock), we recover by taking ownership of the inner value.
        let mut model = self.model.write().unwrap_or_else(|poisoned| {
            tracing::warn!("FastEmbed model lock was poisoned, recovering...");
            poisoned.into_inner()
        });

        // Generate embeddings using the mutable reference
        // Note: For timeout protection, wrap calls to this method in tokio::time::timeout
        // at the async call site (e.g., in mcp_server/indexing.rs)
        let embeddings = model
            .embed(texts, None)
            .context("Failed to generate embeddings")?;

        Ok(embeddings)
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn model_name(&self) -> &str {
        &self.model_name
    }
}

impl Default for FastEmbedManager {
    fn default() -> Self {
        Self::new().expect("Failed to initialize default FastEmbed model")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_generation() {
        let manager = FastEmbedManager::new().unwrap();
        let texts = vec![
            "fn main() { println!(\"Hello, world!\"); }".to_string(),
            "pub struct Vector { x: f32, y: f32 }".to_string(),
        ];

        let embeddings = manager.embed_batch(texts).unwrap();
        assert_eq!(embeddings.len(), 2);
        assert_eq!(embeddings[0].len(), 384);
        assert_eq!(embeddings[1].len(), 384);
    }

    #[test]
    fn test_empty_batch() {
        let manager = FastEmbedManager::new().unwrap();
        let embeddings = manager.embed_batch(vec![]).unwrap();
        assert_eq!(embeddings.len(), 0);
    }

    #[test]
    fn test_dimension() {
        let manager = FastEmbedManager::new().unwrap();
        assert_eq!(manager.dimension(), 384);
    }

    #[test]
    fn test_model_name() {
        let manager = FastEmbedManager::new().unwrap();
        assert_eq!(manager.model_name(), "all-MiniLM-L6-v2");
    }

    #[test]
    fn test_default() {
        let manager = FastEmbedManager::default();
        assert_eq!(manager.dimension(), 384);
        assert_eq!(manager.model_name(), "all-MiniLM-L6-v2");
    }

    #[test]
    fn test_single_text() {
        let manager = FastEmbedManager::new().unwrap();
        let texts = vec!["Hello world".to_string()];
        let embeddings = manager.embed_batch(texts).unwrap();
        assert_eq!(embeddings.len(), 1);
        assert_eq!(embeddings[0].len(), 384);
    }

    #[test]
    fn test_large_batch() {
        let manager = FastEmbedManager::new().unwrap();
        let texts: Vec<String> = (0..10).map(|i| format!("Test text {}", i)).collect();
        let embeddings = manager.embed_batch(texts).unwrap();
        assert_eq!(embeddings.len(), 10);
        for embedding in embeddings {
            assert_eq!(embedding.len(), 384);
        }
    }

    #[test]
    fn test_with_model_allminilm_l12() {
        let manager = FastEmbedManager::with_model(EmbeddingModel::AllMiniLML12V2).unwrap();
        assert_eq!(manager.dimension(), 384);
    }

    #[test]
    fn test_with_model_bge_base() {
        let manager = FastEmbedManager::with_model(EmbeddingModel::BGEBaseENV15).unwrap();
        assert_eq!(manager.dimension(), 768);
        assert_eq!(manager.model_name(), "BAAI/bge-base-en-v1.5");
    }

    #[test]
    fn test_with_model_bge_small() {
        let manager = FastEmbedManager::with_model(EmbeddingModel::BGESmallENV15).unwrap();
        assert_eq!(manager.dimension(), 384);
    }

    #[test]
    fn test_from_local_path_missing_dir() {
        let nonexistent = std::path::PathBuf::from("/this/path/does/not/exist/project-rag-test");
        let result = FastEmbedManager::from_local_path(&nonexistent, "all-MiniLM-L6-v2");
        let err = match result {
            Ok(_) => panic!("expected error for missing directory"),
            Err(e) => e,
        };
        let msg = format!("{:#}", err);
        assert!(
            msg.contains(nonexistent.to_string_lossy().as_ref()),
            "error should name the missing directory, got: {}",
            msg
        );
    }

    #[test]
    fn test_from_local_path_missing_onnx() {
        let dir = tempfile::tempdir().unwrap();
        // Write only the four tokenizer files; intentionally omit the ONNX.
        for name in [
            "tokenizer.json",
            "config.json",
            "special_tokens_map.json",
            "tokenizer_config.json",
        ] {
            std::fs::write(dir.path().join(name), b"{}").unwrap();
        }

        let result = FastEmbedManager::from_local_path(dir.path(), "all-MiniLM-L6-v2");
        let err = match result {
            Ok(_) => panic!("expected error when ONNX file is missing"),
            Err(e) => e,
        };
        let msg = format!("{:#}", err);
        assert!(
            msg.contains("model.onnx") && msg.contains("model_quantized.onnx"),
            "error should mention both ONNX filenames, got: {}",
            msg
        );
    }
}
