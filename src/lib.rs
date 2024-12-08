use zed_extension_api::{self as zed, Result, LanguageServerId};

struct ErrorHolmesExtension {
    cached_server_path: Option<String>,
}

impl ErrorHolmesExtension {
    fn detect_language_from_project(worktree: &zed::Worktree) -> Option<String> {
        let files_to_check = vec![
            ("package.json", "JavaScript/TypeScript"),
            ("requirements.txt", "Python"),
            ("Gemfile", "Ruby"),
        ];

        for (file, language) in files_to_check {
            // Usa `which()` para comprobar si el archivo existe
            if worktree.which(file).is_some() {
                return Some(language.to_string());
            }
        }

        None
    }

    fn get_language_server_path(
        &mut self,
        language: &str,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = self.cached_server_path.clone() {
            return Ok(path);
        }

        // Detectar la ruta del servidor LSP según el lenguaje
        let server_path = match language {
            "JavaScript/TypeScript" => "typescript-language-server",  // Ejemplo para TypeScript
            "Python" => "pyls",                                      // Ejemplo para Python
            _ => "default-lsp-server",                               // Caso predeterminado
        };

        let path = worktree.which(server_path).ok_or_else(|| {
            format!("Debe instalar el servidor LSP para el lenguaje '{}'", language)
        })?;

        self.cached_server_path = Some(path.clone());
        Ok(path)
    }
}

impl zed::Extension for ErrorHolmesExtension {
    fn new() -> Self {
        Self {
            cached_server_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let language = Self::detect_language_from_project(worktree).ok_or_else(|| {
            "No se pudo detectar el lenguaje del proyecto".to_string()
        })?;

        let server_path = self.get_language_server_path(&language, worktree)?;

        Ok(zed::Command {
            command: server_path,
            args: vec!["--stdio".to_string()], // Argumentos según el servidor
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _: &LanguageServerId,
        _: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(serde_json::json!({
            "provideFormatter": true,
            "diagnostics": true,  // Activar diagnósticos como errores y advertencias
        })))
    }
}

zed::register_extension!(ErrorHolmesExtension);
