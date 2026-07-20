use tauri::State;

use crate::{
    models::note::Note,
    repositories::note_repository::NoteRepository,
};

#[tauri::command]
pub async fn create_note(
    title: String,
    content: String,
    repository: State<'_, NoteRepository>,
) -> Result<i64, String> {
    let title = title.trim();
    let content = content.trim();

    if title.is_empty() {
        return Err("タイトルを入力してください。".to_string());
    }

    if content.is_empty() {
        return Err("内容を入力してください。".to_string());
    }

    repository
        .create(title, content)
        .await
        .map_err(|error| {
            eprintln!("メモ保存エラー: {error}");
            "メモの保存に失敗しました。".to_string()
        })
}

#[tauri::command]
pub async fn get_notes(
    repository: State<'_, NoteRepository>,
) -> Result<Vec<Note>, String> {
    repository.find_all().await.map_err(|error| {
        eprintln!("メモ一覧取得エラー: {error}");
        "メモ一覧の取得に失敗しました。".to_string()
    })
}

#[tauri::command]
pub async fn get_note(
    id: i64,
    repository: State<'_, NoteRepository>,
) -> Result<Option<Note>, String> {
    repository.find_by_id(id).await.map_err(|error| {
        eprintln!("メモ取得エラー: {error}");
        "メモの取得に失敗しました。".to_string()
    })
}

#[tauri::command]
pub async fn update_note(
    id: i64,
    title: String,
    content: String,
    repository: State<'_, NoteRepository>,
) -> Result<bool, String> {
    let title = title.trim();
    let content = content.trim();

    if title.is_empty() {
        return Err("タイトルを入力してください。".to_string());
    }

    if content.is_empty() {
        return Err("内容を入力してください。".to_string());
    }

    repository
        .update(id, title, content)
        .await
        .map_err(|error| {
            eprintln!("メモ更新エラー: {error}");
            "メモの更新に失敗しました。".to_string()
        })
}

#[tauri::command]
pub async fn delete_note(
    id: i64,
    repository: State<'_, NoteRepository>,
) -> Result<bool, String> {
    repository.delete(id).await.map_err(|error| {
        eprintln!("メモ削除エラー: {error}");
        "メモの削除に失敗しました。".to_string()
    })
}