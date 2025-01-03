use std::collections::HashMap;
use zed_extension_api::serde_json::Value;
use zed_extension_api::{settings::CommandSettings, settings::LspSettings, Worktree};

pub struct MyLspExtension;

// Aquí configuramos el servidor LSP, por ejemplo, para `rust-analyzer`
impl MyLspExtension {
    pub fn new(worktree: &Worktree) -> MyLspExtension {
        // Configuración del servidor LSP
        let settings = get_lsp_settings("rust-analyzer", worktree).unwrap();

        // Inicializa el servidor LSP
        MyLspExtension // Devuelve la extensión configurada
    }

    // Esta función maneja los mensajes LSP, específicamente los diagnósticos
    pub fn handle_lsp_response(response: Value) {
        // Procesar la respuesta LSP que contiene los diagnósticos
        // Aquí verificarías si la respuesta contiene información relevante de diagnóstico
        if let Some(diagnostics) = response.get("diagnostics") {
            // Aquí podrías recorrer los diagnósticos y mostrarlos de alguna forma
            // Este es un ejemplo simplificado
            for diagnostic in diagnostics.as_array().unwrap() {
                println!("Error: {}", diagnostic["message"].as_str().unwrap());
            }
        }
    }
}

// Función para obtener la configuración del servidor LSP
fn get_lsp_settings(
    language_server_name: &str,
    worktree: &Worktree,
) -> Result<LspSettings, Box<dyn std::error::Error>> {
    let mut env = HashMap::new();
    env.insert("RUST_LOG".to_string(), "debug".to_string()); // Ejemplo de variable de entorno

    let settings = LspSettings {
        binary: Some(CommandSettings {
            path: Some(language_server_name.to_string()), // Ruta al binario del servidor LSP
            arguments: Some(vec![]), // Aquí puedes añadir los argumentos necesarios para el servidor
            env: Some(env),          // Variables de entorno necesarias
        }),
        initialization_options: Some(Value::Object(zed_extension_api::serde_json::Map::new())), // Opciones de inicialización
        settings: Some(Value::Object(zed_extension_api::serde_json::Map::new())), // Configuración adicional
    };

    Ok(settings)
}
