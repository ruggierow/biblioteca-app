use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Livro {
    titulo: String,
    autor: String,
    tema: String,
    ano: i32,
    emprestado: bool,
    #[serde(default)]
    comentarios: String,
    #[serde(default)]
    local: String,
    #[serde(default)]
    grupo_literatura: bool,
}

fn caminho_arquivo() -> PathBuf {
    if let Ok(p) = std::env::var("BIBLIOTECA_FILE") {
        return PathBuf::from(p);
    }
    let home = home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    let em_docs     = home.join("Documents/biblioteca.txt");
    let em_docs_old = home.join("Documents-old/biblioteca.txt");
    if em_docs.exists()     { return em_docs; }
    if em_docs_old.exists() { return em_docs_old; }
    if home.join("Documents").exists()     { return em_docs; }
    if home.join("Documents-old").exists() { return em_docs_old; }
    home.join("biblioteca.txt")
}

#[tauri::command]
fn obter_caminho_arquivo() -> String {
    caminho_arquivo().to_string_lossy().to_string()
}

#[tauri::command]
fn carregar_biblioteca() -> Vec<Livro> {
    let conteudo = match fs::read_to_string(caminho_arquivo()) {
        Ok(c) => c,
        Err(_) => return vec![],
    };
    conteudo
        .lines()
        .filter_map(|linha| {
            let c: Vec<&str> = linha.split('\t').collect();
            if c.len() < 5 { return None; }
            Some(Livro {
                titulo:          c[0].to_string(),
                autor:           c[1].to_string(),
                tema:            c[2].to_string(),
                ano:             c[3].parse().unwrap_or(0),
                emprestado:      c[4] == "1",
                comentarios:     c.get(5).unwrap_or(&"").to_string(),
                local:           c.get(6).unwrap_or(&"").to_string(),
                grupo_literatura: c.get(7).map(|&s| s == "1").unwrap_or(false),
            })
        })
        .collect()
}

#[tauri::command]
fn fazer_backup(sufixo: String) -> Result<String, String> {
    let origem = caminho_arquivo();
    if !origem.exists() {
        return Err("Arquivo biblioteca.txt não encontrado.".to_string());
    }
    let pasta = origem.parent().unwrap_or_else(|| std::path::Path::new("."));
    let destino = pasta.join(format!("biblioteca_{}.txt", sufixo));
    fs::copy(&origem, &destino).map_err(|e| e.to_string())?;
    Ok(destino.to_string_lossy().to_string())
}

#[tauri::command]
fn salvar_biblioteca(livros: Vec<Livro>) -> Result<(), String> {
    let conteudo: String = livros
        .iter()
        .map(|l| {
            format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                l.titulo,
                l.autor,
                l.tema,
                l.ano,
                if l.emprestado { "1" } else { "0" },
                l.comentarios,
                l.local,
                if l.grupo_literatura { "1" } else { "0" },
            )
        })
        .collect();
    fs::write(caminho_arquivo(), conteudo).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            carregar_biblioteca,
            salvar_biblioteca,
            obter_caminho_arquivo,
            fazer_backup,
        ])
        .run(tauri::generate_context!())
        .expect("erro ao iniciar a aplicação");
}
