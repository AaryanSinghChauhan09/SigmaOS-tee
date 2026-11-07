//! Tee model.

#[derive(Debug, Clone)]
pub struct TeeOutput {
    pub stdout: String,
    pub files: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct Tee {
    pub append: bool,
}

impl Default for Tee {
    fn default() -> Self {
        Self { append: false }
    }
}

impl Tee {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tee(&self, input: &str, file_paths: &[&str]) -> TeeOutput {
        let mut files = vec![];
        
        for path in file_paths {
            files.push((path.to_string(), input.to_string()));
        }
        
        TeeOutput {
            stdout: input.to_string(),
            files,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tee() {
        let t = Tee::new();
        let out = t.tee("hello world", &["file1.txt", "file2.txt"]);
        assert_eq!(out.stdout, "hello world");
        assert_eq!(out.files.len(), 2);
    }
}
